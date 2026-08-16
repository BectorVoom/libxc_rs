//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1676/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1676(t12690: f64, t1284: f64, t3601: f64, t3727: f64, t1209: f64, t17948: f64, t12050: f64, t471: f64, t1214: f64, t3588: f64, t12699: f64, t12714: f64, t12744: f64, t1280: f64, t1288: f64, t13121: f64, t13148: f64, t13149: f64, t13153: f64, t17888: f64, t17949: f64, t3666: f64, t3670: f64, t3767: f64, t3769: f64, t3774: f64, t44501: f64, t44585: f64, t44944: f64, t45391: f64, t45584: f64) -> (f64, f64) {
    let t45726 = t12690 * t1284;
    let t45734 = t3727 * t3601;
    let t45738 = t1209 * t17948;
    let t45739 = t12050 * t471;
    let t45740 = t45739 * t1214;
    let t45744 = t45739 * t3588;
    let t45760 = 0.26341796731742046395e1_f64 * t45726 * t1288 - 0.79025390195226139183e1_f64 * t12744 * t13153 + 0.15805078039045227836e2_f64 * t13148 * t45584 * t13149 + 0.79025390195226139183e1_f64 * t3767 * t45734 * t3769 - 0.26341796731742046395e1_f64 * t45738 * t44501 * t45740 + 0.39512695097613069592e1_f64 * t17949 * t44585 * t45744 + 0.15805078039045227836e2_f64 * t17888 * t12714 + 0.52683593463484092788e1_f64 * t3670 * t1280 * t44944 + 0.79025390195226139183e1_f64 * t12699 * t3774 - 0.79025390195226139183e1_f64 * t3666 * t13121 + 0.39512695097613069591e1_f64 * t3670 * t1280 * t45391;
    (t45734, t45760)
}
