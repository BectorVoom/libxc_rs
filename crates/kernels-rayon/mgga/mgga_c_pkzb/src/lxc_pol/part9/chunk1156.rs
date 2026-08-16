//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1156/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1156(t1037: f64, t16406: f64, t16369: f64, t6924: f64, t1634: f64, t1692: f64, t17009: f64, t1733: f64, t179: f64, t19867: f64, t20114: f64, t20118: f64, t20121: f64, t20127: f64, t20137: f64, t20141: f64, t2592: f64, t2645: f64, t2653: f64, t2660: f64, t2661: f64, t50: f64, t51: f64, t5217: f64, t5244: f64, t5279: f64, t568: f64, t580: f64, t581: f64, t612: f64, t6853: f64, t6864: f64, t6944: f64, t6970: f64, t6990: f64, t6999: f64) -> f64 {
    let t20155 = t16406 * t1037;
    let t20157 = t16369 * t6924;
    let t20163 = 0.25724410870841842183e-2_f64 * t1733 * t179 * t6970 * t1692 + 0.30011812682648815881e-2_f64 * t2592 * t179 * t20114 + 0.24009450146119052704e-1_f64 * t20118 - 0.60023625365297631762e-2_f64 * t17009 + 7.0_f64 / 48.0_f64 * t20121 - t580 * t581 * t50 * t19867 / 48.0_f64 - 0.60023625365297631762e-1_f64 * t20127 + 0.12862205435420921092e-1_f64 * t612 * t2660 * t6999 * t1692 - 0.77173232612525526549e-1_f64 * t612 * t6990 * t6999 * t1634 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t20137 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t2653 * t20141 - 0.10289764348336736873e-1_f64 * t5244 * t179 * t6864 * t6944 + 0.12862205435420921092e-1_f64 * t612 * t2660 * t51 * t6853 * t568 + 0.37792653007779990369e-1_f64 * t20155 + 7.0_f64 / 4.0_f64 * t20157 + 0.42874018118069736972e-2_f64 * t612 * t2660 * t2661 * t5217;
    t20163
}
