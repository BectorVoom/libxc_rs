//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1676/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1676<F: Float>(t12690: F, t1284: F, t3601: F, t3727: F, t1209: F, t17948: F, t12050: F, t471: F, t1214: F, t3588: F, t12699: F, t12714: F, t12744: F, t1280: F, t1288: F, t13121: F, t13148: F, t13149: F, t13153: F, t17888: F, t17949: F, t3666: F, t3670: F, t3767: F, t3769: F, t3774: F, t44501: F, t44585: F, t44944: F, t45391: F, t45584: F) -> (F, F) {
    let t45726 = t12690 * t1284;
    let t45734 = t3727 * t3601;
    let t45738 = t1209 * t17948;
    let t45739 = t12050 * t471;
    let t45740 = t45739 * t1214;
    let t45744 = t45739 * t3588;
    let t45760 = F::cast_from(0.26341796731742046395e1_f64) * t45726 * t1288 - F::cast_from(0.79025390195226139183e1_f64) * t12744 * t13153 + F::cast_from(0.15805078039045227836e2_f64) * t13148 * t45584 * t13149 + F::cast_from(0.79025390195226139183e1_f64) * t3767 * t45734 * t3769 - F::cast_from(0.26341796731742046395e1_f64) * t45738 * t44501 * t45740 + F::cast_from(0.39512695097613069592e1_f64) * t17949 * t44585 * t45744 + F::cast_from(0.15805078039045227836e2_f64) * t17888 * t12714 + F::cast_from(0.52683593463484092788e1_f64) * t3670 * t1280 * t44944 + F::cast_from(0.79025390195226139183e1_f64) * t12699 * t3774 - F::cast_from(0.79025390195226139183e1_f64) * t3666 * t13121 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t1280 * t45391;
    (t45734, t45760)
}
