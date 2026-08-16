//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 322/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk322(t4299: f64, t871: f64, t296: f64, t1255: f64, t824: f64, t840: f64, t1901: f64, t2816: f64, t2817: f64, t2819: f64, t4248: f64, t4252: f64, t4257: f64, t4262: f64, t4267: f64, t4271: f64, t4273: f64, t4276: f64, t4280: f64, t4283: f64, t446: f64) -> f64 {
    let t4300 = t871 * t4299;
    let t4301 = t296 * t4300;
    let t4305 = t840 * t1255 * t824;
    let t4308 = -t446 * t4248 / 3.0_f64 - t446 * t4252 / 3.0_f64 + t1901 * t4257 / 9.0_f64 + t1901 * t4262 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t4267 + t2816 - t4271 / 9.0_f64 + t4273 / 9.0_f64 - t446 * t4276 / 3.0_f64 - t446 * t4280 / 3.0_f64 + t4283 / 27.0_f64 + t2817 / 9.0_f64 + t2819 / 9.0_f64 - t446 * t4301 / 3.0_f64 - t446 * t4305 / 3.0_f64;
    t4308
}
