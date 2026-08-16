//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1626/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1626(t13985: f64, t9816: f64, t5706: f64, t9962: f64, t4000: f64, t820: f64, t844: f64) -> (f64, f64, f64) {
    let t13987 = 0.10164000561857065645e-3_f64 * t9816 * t13985;
    let t13988 = t9962 * t5706;
    let t13999 = t820 * t4000 * t844;
    (t13987, t13988, t13999)
}
