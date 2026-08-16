//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1270/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1270(t1399: f64, t1872: f64, t9818: f64, t9816: f64, t5706: f64, t9962: f64, t4000: f64, t820: f64, t844: f64, t5677: f64, t13847: f64, t13848: f64) -> (f64, f64, f64, f64, f64) {
    let t13985 = t9818 * t1872 * t1399;
    let t13987 = 0.10164000561857065645e-3_f64 * t9816 * t13985;
    let t13988 = t9962 * t5706;
    let t13999 = t820 * t4000 * t844;
    let t14001 = 0.40015750243531754508e-2_f64 * t13999 * t5677;
    let t14005 = t13847 * t13848 * t1399;
    (t13985, t13987, t13988, t14001, t14005)
}
