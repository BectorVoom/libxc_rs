//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1156/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1156<F: Float>(t1037: F, t16406: F, t16369: F, t6924: F, t1634: F, t1692: F, t17009: F, t1733: F, t179: F, t19867: F, t20114: F, t20118: F, t20121: F, t20127: F, t20137: F, t20141: F, t2592: F, t2645: F, t2653: F, t2660: F, t2661: F, t50: F, t51: F, t5217: F, t5244: F, t5279: F, t568: F, t580: F, t581: F, t612: F, t6853: F, t6864: F, t6944: F, t6970: F, t6990: F, t6999: F) -> F {
    let t20155 = t16406 * t1037;
    let t20157 = t16369 * t6924;
    let t20163 = F::new(0.25724410870841842183e-2) * t1733 * t179 * t6970 * t1692 + F::new(0.30011812682648815881e-2) * t2592 * t179 * t20114 + F::new(0.24009450146119052704e-1) * t20118 - F::new(0.60023625365297631762e-2) * t17009 + F::new(7.0) / F::new(48.0) * t20121 - t580 * t581 * t50 * t19867 / F::new(48.0) - F::new(0.60023625365297631762e-1) * t20127 + F::new(0.12862205435420921092e-1) * t612 * t2660 * t6999 * t1692 - F::new(0.77173232612525526549e-1) * t612 * t6990 * t6999 * t1634 - F::new(0.64311027177104605458e-3) * t2645 * t179 * t20137 - F::new(0.12862205435420921092e-1) * t5279 * t179 * t2653 * t20141 - F::new(0.10289764348336736873e-1) * t5244 * t179 * t6864 * t6944 + F::new(0.12862205435420921092e-1) * t612 * t2660 * t51 * t6853 * t568 + F::new(0.37792653007779990369e-1) * t20155 + F::new(7.0) / F::new(4.0) * t20157 + F::new(0.42874018118069736972e-2) * t612 * t2660 * t2661 * t5217;
    t20163
}
