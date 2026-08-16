//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1397/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1397(t11025: f64, t2435: f64, t10981: f64, t588: f64, t780: f64, t10991: f64, t39497: f64, t787: f64, t788: f64, t2448: f64, t9292: f64, t11036: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40994 = t2435 * t11025;
    let t40998 = 0.15709759505761725819e-2_f64 * t10981 * t780 * t588;
    let t40999 = t2435 * t10991;
    let t41003 = 0.10118827226026589797e0_f64 * t787 * t788 * t39497;
    let t41004 = t9292 * t2448;
    let t41006 = t2435 * t11036;
    (t40994, t40998, t40999, t41003, t41004, t41006)
}
