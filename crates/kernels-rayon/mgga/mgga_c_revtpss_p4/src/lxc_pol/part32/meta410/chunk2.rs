//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1423/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1423(t19021: f64, t964: f64, t973: f64, t981: f64, t3022: f64, t6227: f64, t11528: f64, t6110: f64, t2869: f64, t6142: f64, t11134: f64, t11560: f64, t15189: f64, t15483: f64, t15484: f64, t15485: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64, f64, f64) {
    let t19023 = t964 * t19021 * t973;
    let t19025 = 0.5848223622634646207e0_f64 * t981 * t19023;
    let t19027 = 0.17315859105681463759e2_f64 * t3022 * t6227;
    let t19029 = 2.0_f64 * t11528 * t6110;
    let t19031 = 1.0_f64 * t2869 * t6142;
    let t19045 = -t11560 - 0.41203703703703703703e-2_f64 * t11134 - 0.82407407407407407408e-2_f64 * t15189 + t15483 - t15484 + t15485 + 0.20601851851851851852e-2_f64 * t18919 - 0.10300925925925925926e-1_f64 * t18906 + 0.37083333333333333333e-1_f64 * t18911 - 0.12361111111111111111e-1_f64 * t18915 - 0.61805555555555555557e-2_f64 * t18924 - 0.55625000000000000001e-1_f64 * t18928 + 0.37083333333333333334e-1_f64 * t18932 + 0.30902777777777777778e-2_f64 * t18934 - 0.61805555555555555555e-2_f64 * t18939 + 0.18541666666666666667e-1_f64 * t18944 - 0.92708333333333333333e-2_f64 * t18948;
    (t19025, t19027, t19029, t19031, t19045)
}
