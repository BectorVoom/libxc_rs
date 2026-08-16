//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 639/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk639(t2841: f64, t2895: f64, t141: f64, t1038: f64, t2846: f64, t2850: f64, t2836: f64, t2843: f64, t2848: f64, t2852: f64, t2870: f64, t2878: f64, t2880: f64, t2886: f64, t2888: f64, t2892: f64, t2893: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2896 = t2895 * t2841;
    let t2897 = t141 * t2896;
    let t2899 = t1038 * t2846;
    let t2900 = t141 * t2899;
    let t2902 = t1038 * t2850;
    let t2903 = t141 * t2902;
    let t2905 = -0.9494625e0_f64 * t2870 + 0.1898925e1_f64 * t2878 + t2880 - 0.19931111111111111111e0_f64 * t2836 - 0.19931111111111111111e0_f64 * t2843 + 0.59793333333333333334e0_f64 * t2848 + 0.29896666666666666667e0_f64 * t2852 + 0.15358125e0_f64 * t2886 + 0.3071625e0_f64 * t2888 + t2892 - 0.10954222222222222222e0_f64 * t2893 - 0.27385555555555555556e-1_f64 * t2897 + 0.16431333333333333333e0_f64 * t2900 + 0.82156666666666666667e-1_f64 * t2903;
    (t2896, t2897, t2899, t2900, t2902, t2903, t2905)
}
