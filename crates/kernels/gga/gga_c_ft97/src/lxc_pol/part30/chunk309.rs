//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 309/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk309<F: Float>(t4299: F, t871: F, t296: F, t1255: F, t824: F, t840: F, t1901: F, t2816: F, t2817: F, t2819: F, t4248: F, t4252: F, t4257: F, t4262: F, t4267: F, t4271: F, t4273: F, t4276: F, t4280: F, t4283: F, t446: F) -> (F,) {
    let t4300 = t871 * t4299;
    let t4301 = t296 * t4300;
    let t4305 = t840 * t1255 * t824;
    let t4308 = -t446 * t4248 / 3.0 - t446 * t4252 / 3.0 + t1901 * t4257 / 9.0 + t1901 * t4262 / 9.0 + 2.0 / 9.0 * t1901 * t4267 + t2816 - t4271 / 9.0 + t4273 / 9.0 - t446 * t4276 / 3.0 - t446 * t4280 / 3.0 + t4283 / 27.0 + t2817 / 9.0 + t2819 / 9.0 - t446 * t4301 / 3.0 - t446 * t4305 / 3.0;
    (t4308,)
}
