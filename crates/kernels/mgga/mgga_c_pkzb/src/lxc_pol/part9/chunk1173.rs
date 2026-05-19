//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1173/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1173<F: Float>(t164: F, t1692: F, t17000: F, t17034: F, t1721: F, t1733: F, t179: F, t20075: F, t20141: F, t20242: F, t20252: F, t20262: F, t20263: F, t20265: F, t20267: F, t20272: F, t20275: F, t20398: F, t2593: F, t2600: F, t2645: F, t2646: F, t2661: F, t51: F, t5181: F, t5244: F, t5250: F, t5279: F, t590: F, t592: F, t600: F, t612: F, t6961: F) -> F {
    let t20404 = -F::cast_from(0.51448821741683684367e-2_f64) * t5244 * t179 * t2593 * t1692 * t600 - F::cast_from(0.51448821741683684367e-2_f64) * t5244 * t179 * t6961 * t5250 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t179 * t2600 * t17000 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t2646 * t20141 + F::cast_from(0.68026775414003982662e-1_f64) * t20242 + F::cast_from(0.25724410870841842183e-1_f64) * t17034 * t179 * t2600 * t5181 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t179 * t20075 * t164 - F::cast_from(0.21437009059034868486e-3_f64) * t2645 * t179 * t20252 - F::cast_from(0.51448821741683684367e-2_f64) * t5244 * t179 * t20075 * t1721 - t20262 - F::cast_from(0.60023625365297631762e-2_f64) * t20263 - F::cast_from(0.12004725073059526352e0_f64) * t20265 + F::cast_from(0.18007087609589289528e0_f64) * t612 * t20267 * t2661 * t5181 + F::cast_from(0.36014175219178579057e0_f64) * t20272 - t20275 - F::cast_from(0.21437009059034868486e-3_f64) * t590 * t592 * t51 * t20398 * t164;
    t20404
}
