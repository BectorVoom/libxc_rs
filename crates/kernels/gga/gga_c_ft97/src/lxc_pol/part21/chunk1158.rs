//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1158/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1158<F: Float>(t100305: F, t102119: F, t102120: F, t102121: F, t102122: F, t116444: F, t116448: F, t116453: F, t116456: F, t116460: F, t116463: F, t116467: F, t15625: F, t1564: F, t446: F, t5691: F) -> (F, F) {
    let t116469 = -2.0 / 9.0 * t116444 - 2.0 / 3.0 * t116448 - 2.0 / 9.0 * t100305 - t102119 - t116453 / 3.0 + t116456 / 9.0 - t116460 / 18.0 - t116463 - t116467 / 2.0 + t102120 + t102121 - t102122;
    let t116473 = t446 * t1564 * t5691 * t15625;
    (t116469, t116473)
}
