//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 955/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk955(t3006: f64, t974: f64, t3014: f64, t972: f64, t2873: f64, t910: f64, t2876: f64, t11300: f64, t935: f64, t2924: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> (f64, f64, f64, f64, f64) {
    let t11521 = t974 * t3006;
    let t11524 = t3006 * t3014;
    let t11525 = t11524 * t972;
    let t11528 = t910 * t2873;
    let t11530 = 6.0_f64 * t11528 * t2876;
    let t11531 = t11300 * t935;
    let t11533 = 6.0_f64 * t2924 * t11531;
    let t11534 = 0.55403703703703703703e-1_f64 * t11132;
    let t11545 = -t11534 - 0.23744444444444444444e-1_f64 * t11134 + 0.11872222222222222222e-1_f64 * t11136 - 0.35616666666666666666e-1_f64 * t11138 + 0.17808333333333333333e-1_f64 * t11140 - 0.19787037037037037037e-1_f64 * t11147 + 0.71233333333333333332e-1_f64 * t11153 - 0.35616666666666666666e-1_f64 * t11158 - 0.10685e0_f64 * t11162 + 0.10685e0_f64 * t11167 - 0.17808333333333333333e-1_f64 * t11171;
    (t11521, t11525, t11530, t11533, t11545)
}
