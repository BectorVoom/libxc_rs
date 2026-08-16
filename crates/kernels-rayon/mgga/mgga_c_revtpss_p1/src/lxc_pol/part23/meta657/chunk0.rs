//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2386/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2386(t11003: f64, t9303: f64, t10981: f64, t22: f64, t868: f64, t886: f64, t2445: f64, t9292: f64, t588: f64, t780: f64, t39497: f64, t787: f64, t788: f64) -> (f64, f64, f64, f64, f64) {
    let t40970 = t9303 * t11003;
    let t40978 = t10981 * t868 * t22 * t886;
    let t40988 = t9292 * t2445;
    let t40998 = 0.15709759505761725819e-2_f64 * t10981 * t780 * t588;
    let t41003 = 0.10118827226026589797e0_f64 * t787 * t788 * t39497;
    (t40970, t40978, t40988, t40998, t41003)
}
