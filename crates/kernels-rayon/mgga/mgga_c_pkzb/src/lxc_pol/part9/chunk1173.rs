//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1173/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1173(t164: f64, t1692: f64, t17000: f64, t17034: f64, t1721: f64, t1733: f64, t179: f64, t20075: f64, t20141: f64, t20242: f64, t20252: f64, t20262: f64, t20263: f64, t20265: f64, t20267: f64, t20272: f64, t20275: f64, t20398: f64, t2593: f64, t2600: f64, t2645: f64, t2646: f64, t2661: f64, t51: f64, t5181: f64, t5244: f64, t5250: f64, t5279: f64, t590: f64, t592: f64, t600: f64, t612: f64, t6961: f64) -> f64 {
    let t20404 = -0.51448821741683684367e-2_f64 * t5244 * t179 * t2593 * t1692 * t600 - 0.51448821741683684367e-2_f64 * t5244 * t179 * t6961 * t5250 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t2600 * t17000 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t2646 * t20141 + 0.68026775414003982662e-1_f64 * t20242 + 0.25724410870841842183e-1_f64 * t17034 * t179 * t2600 * t5181 + 0.85748036236139473944e-3_f64 * t1733 * t179 * t20075 * t164 - 0.21437009059034868486e-3_f64 * t2645 * t179 * t20252 - 0.51448821741683684367e-2_f64 * t5244 * t179 * t20075 * t1721 - t20262 - 0.60023625365297631762e-2_f64 * t20263 - 0.12004725073059526352e0_f64 * t20265 + 0.18007087609589289528e0_f64 * t612 * t20267 * t2661 * t5181 + 0.36014175219178579057e0_f64 * t20272 - t20275 - 0.21437009059034868486e-3_f64 * t590 * t592 * t51 * t20398 * t164;
    t20404
}
