//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 938/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk938(t35551: f64, t9222: f64, t1679: f64, t7900: f64, t5016: f64, t8404: f64, t4601: f64, t8407: f64, t5055: f64, t7444: f64, t236: f64, t321: f64, t3351: f64, t35312: f64, t9211: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40621 = t9222 * t35551;
    let t40623 = t1679 * t7900;
    let t40625 = t5016 * t8404;
    let t40627 = t4601 * t8407;
    let t40630 = t5055 * t7444;
    let t40637 = t3351 * t35312 * t236 * t9211 * t321;
    (t40621, t40623, t40625, t40627, t40630, t40637)
}
