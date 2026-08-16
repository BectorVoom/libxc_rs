//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 647/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk647(t1081: f64, t2975: f64, t2834: f64, t2891: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t2870: f64, t2878: f64, t2886: f64, t2888: f64, t2893: f64, t2897: f64, t2900: f64, t2903: f64) -> (f64, f64, f64, f64) {
    let t2976 = t2975 * t1081;
    let t2981 = 0.40256666666666666667e0_f64 * t2834;
    let t2988 = 0.137975e0_f64 * t2891;
    let t2993 = -0.1294625e1_f64 * t2870 + 0.258925e1_f64 * t2878 + t2981 - 0.20128333333333333334e0_f64 * t2836 - 0.20128333333333333333e0_f64 * t2843 + 0.60385e0_f64 * t2848 + 0.301925e0_f64 * t2852 + 0.82524375e-1_f64 * t2886 + 0.16504875e0_f64 * t2888 + t2988 - 0.11038e0_f64 * t2893 - 0.27595e-1_f64 * t2897 + 0.16557e0_f64 * t2900 + 0.82785e-1_f64 * t2903;
    (t2976, t2981, t2988, t2993)
}
