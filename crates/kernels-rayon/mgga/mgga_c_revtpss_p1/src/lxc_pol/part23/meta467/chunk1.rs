//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1912/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1912(t11134: f64, t11890: f64, t15189: f64, t15874: f64, t15875: f64, t15876: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> f64 {
    let t19855 = -t11890 - 0.37037037037037037037e-2_f64 * t11134 - 0.74074074074074074074e-2_f64 * t15189 + t15874 - t15875 + t15876 + 0.18518518518518518518e-2_f64 * t18919 - 0.92592592592592592592e-2_f64 * t18906 + 0.33333333333333333333e-1_f64 * t18911 - 0.11111111111111111111e-1_f64 * t18915 - 0.55555555555555555557e-2_f64 * t18924 - 0.50000000000000000001e-1_f64 * t18928 + 0.33333333333333333334e-1_f64 * t18932 + 0.27777777777777777778e-2_f64 * t18934 - 0.55555555555555555555e-2_f64 * t18939 + 0.16666666666666666667e-1_f64 * t18944 - 0.83333333333333333333e-2_f64 * t18948;
    t19855
}
