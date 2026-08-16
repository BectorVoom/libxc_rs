//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 861/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk861(t35: f64, t37640: f64, t11109: f64, t11351: f64, t1594: f64, t1603: f64, t1617: f64, t1626: f64, t1631: f64, t1633: f64, t1656: f64, t1657: f64, t1660: f64, t1683: f64, t1685: f64, t1687: f64, t1701: f64, t1712: f64, t2021: f64, t2035: f64, t2037: f64, t22834: f64, t3019: f64, t3020: f64, t372: f64, t374: f64, t37611: f64, t37614: f64, t37622: f64, t37628: f64, t399: f64, t401: f64, t5545: f64, t7202: f64, t77: f64, t7833: f64, t7877: f64, t7879: f64, t7919: f64, t7977: f64, t7982: f64, t7993: f64, t8018: f64, t8070: f64, t8174: f64) -> (f64, f64) {
    let t37641 = t37640 * t35;
    let t37666 = -0.279058811357253504e0_f64 * t22834 * t8174 + 0.279058811357253504e0_f64 * t37611 * t1626 + 0.22023512095983737145e1_f64 * t5545 * t1701 * t37614 * t401 + 0.23238868087529279928e-2_f64 * t7919 * t1660 - 0.40559281352147498558e-3_f64 * t7982 * t37622 - 0.139529405678626752e0_f64 * t7919 * t1657 + 0.69764702839313376e-2_f64 * t372 * t1631 * t37628 - 0.24335568811288499135e-3_f64 * t11109 * t7993 + 0.27039520901431665705e-3_f64 * t3019 * t3020 * t77 * t7977 + 0.279058811357253504e-1_f64 * t7919 * t1633 + 0.38704743803858356237e-5_f64 * t372 * t2021 * t37641 - 0.139529405678626752e0_f64 * t1603 * t374 * t1656 * t1685 + 0.58097170218823199823e-3_f64 * t372 * t1594 * t37628 + 0.474190451827401039e-1_f64 * t8070 * t399 + 0.279058811357253504e0_f64 * t7877 * t11351 * t7879 + 0.16864243845320605903e-2_f64 * t7202 * t2035 * t2037 * t1712 - 0.45048092923603098705e0_f64 * t1687 * t1683 + 0.26189723469107882512e-2_f64 * t1617 * t7833 * t8018;
    (t37641, t37666)
}
