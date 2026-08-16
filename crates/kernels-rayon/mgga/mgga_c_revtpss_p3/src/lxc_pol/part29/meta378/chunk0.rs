//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1351/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1351(t3936: f64, t5674: f64, t9810: f64, t125: f64, t5591: f64, t1399: f64, t4057: f64, t5704: f64, t1872: f64, t9818: f64, t9816: f64, t5706: f64, t9962: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13967 = t3936 * t5674 * t9810;
    let t13975 = t125 * t5591;
    let t13977 = t3936 * t13975 * t1399;
    let t13981 = t3936 * t5704 * t4057;
    let t13985 = t9818 * t1872 * t1399;
    let t13987 = 0.10164000561857065645e-3_f64 * t9816 * t13985;
    let t13988 = t9962 * t5706;
    (t13967, t13977, t13981, t13985, t13987, t13988)
}
