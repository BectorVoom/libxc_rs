//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2715/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715(t49966: f64, t10600: f64, t18259: f64, t14325: f64, t14390: f64, t14468: f64, t1544: f64, t2403: f64, t2404: f64, t39783: f64, t41197: f64, t49950: f64, t49956: f64, t49958: f64, t49959: f64, t49964: f64, t775: f64) -> (f64, f64, f64, f64) {
    let t49967 = 0.17544670867903938621e1_f64 * t49966;
    let t49969 = 36.0_f64 * t18259 * t10600;
    let t49971 = 72.0_f64 * t14325 * t14390;
    let t49972 = 9.0_f64 * t14468 * t2403 * t2404 + 3.0_f64 * t1544 * t2403 * t41197 + 9.0_f64 * t2403 * t49950 * t775 - t39783 + t49956 - t49958 + t49959 - t49964 - t49967 + t49969 + t49971;
    (t49967, t49969, t49971, t49972)
}
