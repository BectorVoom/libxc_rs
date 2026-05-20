//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1673/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1673<F: Float>(t42859: F, t44531: F, t460: F, t43351: F, t44535: F, t13107: F, t473: F, t1209: F, t17879: F, t44332: F, t487: F, t1214: F, t1234: F, t12690: F, t12706: F, t12751: F, t12756: F, t12757: F, t1291: F, t13118: F, t13134: F, t17880: F, t3666: F, t3746: F, t3755: F, t3769: F, t3782: F, t3783: F, t44431: F, t44759: F, t45609: F, t5458: F) -> (F, F) {
    let t45618 = t42859 * t44531;
    let t45619 = t460 * t45618;
    let t45620 = t43351 * t44535;
    let t45624 = t473 * t13107;
    let t45634 = t1209 * t17879;
    let t45648 = t487 * t44332;
    let t45652 = F::cast_from(0.15805078039045227836e2_f64) * t45619 * t45609 * t45620 - F::cast_from(0.26341796731742046395e1_f64) * t1234 * t45624 * t1214 + F::cast_from(0.79025390195226139183e1_f64) * t3746 * t13118 - F::cast_from(0.79025390195226139183e1_f64) * t3666 * t13134 - F::cast_from(0.79025390195226139184e1_f64) * t17880 * t12706 + F::cast_from(0.79025390195226139183e1_f64) * t45634 * t12757 + F::cast_from(0.26341796731742046395e1_f64) * t12690 * t1291 - F::cast_from(0.26341796731742046395e1_f64) * t3755 * t44431 * t5458 - F::cast_from(0.79025390195226139183e1_f64) * t12751 * t44759 * t3769 + F::cast_from(0.39512695097613069592e1_f64) * t12756 * t44759 * t3783 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t45648 * t3783;
    (t45648, t45652)
}
