//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 412/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk412(t146: f64, t20: f64, t2861: f64, t2: f64, t816: f64, t952: f64, t2864: f64, t2867: f64, t2869: f64, t15: f64, t2863: f64, t2866: f64, t818: f64, t947: f64) -> (f64, f64, f64, f64) {
    let t3092 = t2861 * t146 * t20;
    let t3096 = t816 * t952 * t2;
    let t3104 = -0.44044444444444444445e-2_f64 * t2864 + 0.88088888888888888889e-2_f64 * t2867 + 0.55033333333333333333e-2_f64 * t2869;
    let t3107 = -t3092 * t2863 / 18.0_f64 - t3096 * t818 / 6.0_f64 + t947 * t2866 / 9.0_f64 + t15 * t3104 / 2.0_f64;
    (t3092, t3096, t3104, t3107)
}
