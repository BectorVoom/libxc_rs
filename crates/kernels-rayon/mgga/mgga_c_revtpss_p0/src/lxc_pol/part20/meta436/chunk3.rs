//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1646/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1646(t1196: f64, t12485: f64, t12500: f64, t3497: f64, t12243: f64, t12415: f64, t12248: f64, t3427: f64, t3436: f64, t1149: f64, t12358: f64, t3384: f64) -> (f64, f64, f64, f64) {
    let t45021 = 0.62337092780453269531e3_f64 * t1196 * t12485 * t3497 * t12500;
    let t45023 = 0.1929837539843104208e3_f64 * t12243 * t12415;
    let t45026 = 0.57895126195293126241e3_f64 * t12248 * t3436 * t3427;
    let t45029 = 8.0_f64 * t3384 * t12358 * t1149;
    (t45021, t45023, t45026, t45029)
}
