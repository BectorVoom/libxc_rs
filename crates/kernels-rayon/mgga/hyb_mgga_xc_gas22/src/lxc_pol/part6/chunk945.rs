//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 945/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk945(t3444: f64, t8604: f64, t3443: f64, t6737: f64, t1347: f64, t2228: f64, t2188: f64, t2189: f64, t3356: f64, t6579: f64, t2236: f64, t3352: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8605 = t8604 * t3444;
    let t8608 = t3443 * t6737;
    let t8611 = t1347 * t2228;
    let t8613 = 2.0_f64 * t2188 * t8611;
    let t8614 = t3356 * t2189;
    let t8616 = 0.96491876992155210402e2_f64 * t6579 * t8614;
    let t8617 = t3352 * t2236;
    (t8605, t8608, t8611, t8613, t8614, t8616, t8617)
}
