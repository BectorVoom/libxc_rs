//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3412/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3412(t19021: f64, t3014: f64, t11404: f64, t11461: f64, t15104: f64, t15238: f64, t15242: f64, t15274: f64, t15277: f64, t15280: f64, t15284: f64, t15406: f64, t19167: f64, t19263: f64, t19307: f64, t19311: f64, t2962: f64, t2968: f64, t2987: f64, t3012: f64, t41756: f64, t4652: f64, t4674: f64, t52809: f64, t52812: f64, t52820: f64, t52825: f64, t6158: f64, t63583: f64, t63586: f64, t63589: f64, t63592: f64, t63596: f64, t972: f64) -> f64 {
    let t64072 = t19021 * t3014;
    let t64101 = -0.23392894490538584828e1_f64 * t2987 * t19167 * t972 + 0.34631718211362927518e2_f64 * t3012 * t64072 * t972 + 0.69263436422725855036e2_f64 * t11461 * t19307 + 0.20508037716432813316e4_f64 * t41756 * t19311 - t63583 - t63586 - t63589 - t63592 - t63596 - 8.0_f64 * t52809 * t4652 - 8.0_f64 * t15104 * t15274 - 4.0_f64 * t15104 * t15277 - 0.38596750796862084161e3_f64 * t52812 * t15280 + 0.12865583598954028054e3_f64 * t52820 * t4674 + 0.12865583598954028054e3_f64 * t15406 * t15284 + 0.64327917994770140268e2_f64 * t15406 * t15238 + 0.4138081033541872024e4_f64 * t52825 * t15242 + 12.0_f64 * t11404 * t19263 + 6.0_f64 * t2968 * t6158 * t2962;
    t64101
}
