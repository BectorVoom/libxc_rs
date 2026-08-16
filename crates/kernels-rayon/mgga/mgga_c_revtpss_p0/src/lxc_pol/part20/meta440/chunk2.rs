//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1673/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673(t42859: f64, t44531: f64, t460: f64, t43351: f64, t44535: f64, t13107: f64, t473: f64, t1209: f64, t17879: f64, t44332: f64, t487: f64, t1214: f64, t1234: f64, t12690: f64, t12706: f64, t12751: f64, t12756: f64, t12757: f64, t1291: f64, t13118: f64, t13134: f64, t17880: f64, t3666: f64, t3746: f64, t3755: f64, t3769: f64, t3782: f64, t3783: f64, t44431: f64, t44759: f64, t45609: f64, t5458: f64) -> (f64, f64) {
    let t45618 = t42859 * t44531;
    let t45619 = t460 * t45618;
    let t45620 = t43351 * t44535;
    let t45624 = t473 * t13107;
    let t45634 = t1209 * t17879;
    let t45648 = t487 * t44332;
    let t45652 = 0.15805078039045227836e2_f64 * t45619 * t45609 * t45620 - 0.26341796731742046395e1_f64 * t1234 * t45624 * t1214 + 0.79025390195226139183e1_f64 * t3746 * t13118 - 0.79025390195226139183e1_f64 * t3666 * t13134 - 0.79025390195226139184e1_f64 * t17880 * t12706 + 0.79025390195226139183e1_f64 * t45634 * t12757 + 0.26341796731742046395e1_f64 * t12690 * t1291 - 0.26341796731742046395e1_f64 * t3755 * t44431 * t5458 - 0.79025390195226139183e1_f64 * t12751 * t44759 * t3769 + 0.39512695097613069592e1_f64 * t12756 * t44759 * t3783 - 0.19756347548806534796e1_f64 * t3782 * t45648 * t3783;
    (t45648, t45652)
}
