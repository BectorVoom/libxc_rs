//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1871/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1871(t19172: f64, t19253: f64, t19293: f64, t19334: f64, t300: f64, t6350: f64, t999: f64, t3269: f64, t342: f64, t6343: f64, t11133: f64, t11134: f64, t15127: f64, t15189: f64, t15638: f64, t15639: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64, f64, f64) {
    let t19337 = t300 * (t19172 + t19253 + t19293 + t19334);
    let t19341 = t6350 * t999;
    let t19342 = t3269 * t19341;
    let t19351 = t342 * t6343;
    let t19380 = -t11133 - 0.65851851851851851853e-2_f64 * t11134 - 0.13170370370370370371e-1_f64 * t15189 + 0.65851851851851851853e-2_f64 * t15127 - t15638 + t15639 + 0.32925925925925925927e-2_f64 * t18919 - 0.16462962962962962963e-1_f64 * t18906 + 0.59266666666666666668e-1_f64 * t18911 - 0.19755555555555555556e-1_f64 * t18915 - 0.9877777777777777778e-2_f64 * t18924 - 0.88900000000000000002e-1_f64 * t18928 + 0.59266666666666666668e-1_f64 * t18932 + 0.4938888888888888889e-2_f64 * t18934 - 0.9877777777777777778e-2_f64 * t18939 + 0.29633333333333333334e-1_f64 * t18944 - 0.14816666666666666667e-1_f64 * t18948;
    (t19337, t19341, t19342, t19351, t19380)
}
