//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 751/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk751(t2528: f64, t565: f64, t2072: f64, t2076: f64, t2104: f64, t2480: f64, t1284: f64, t6590: f64, t220: f64, t186: f64, t548: f64, t2499: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6858 = 2.0_f64 / 15.0_f64 * t565 * t2528;
    let t6860 = 8.0_f64 / 15.0_f64 * t2076 * t2072;
    let t6862 = 4.0_f64 / 15.0_f64 * t2104 * t2480;
    let t6864 = 4.0_f64 / 15.0_f64 * t1284 * t2480;
    let t6865 = -t6590;
    let t6866 = t220 * t6865;
    let t6867 = t186 * t6866;
    let t6869 = 4.0_f64 / 15.0_f64 * t548 * t6867;
    let t6871 = 2.0_f64 / 15.0_f64 * t511 * t2499;
    (t6858, t6860, t6862, t6864, t6865, t6866, t6867, t6869, t6871)
}
