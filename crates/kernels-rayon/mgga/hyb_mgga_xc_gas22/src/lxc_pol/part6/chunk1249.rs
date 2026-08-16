//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1249/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1249(t7488: f64, t9324: f64, t7494: f64, t221: f64, t2631: f64, t3636: f64, t7491: f64, t7485: f64, t1100: f64, t462: f64, t9369: f64, t2813: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25939 = t9324 * t7488;
    let t25941 = t9324 * t7494;
    let t25944 = t3636 * t221 * t2631;
    let t25946 = t9324 * t7491;
    let t25948 = t9324 * t7485;
    let t25951 = t462 * t9369 * t1100;
    let t25954 = t462 * t3636 * t2813;
    (t25939, t25941, t25944, t25946, t25948, t25951, t25954)
}
