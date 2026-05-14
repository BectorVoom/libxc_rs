//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1059/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1059<F: Float>(t5257: F, t6958: F, t1034: F, t164: F, t5367: F, t1753: F, t2639: F, t1020: F, t17067: F, t1733: F, t1734: F, t179: F, t19934: F, t20037: F, t20057: F, t20060: F, t20065: F, t20067: F, t20071: F, t20075: F, t20081: F, t2575: F, t2600: F, t2645: F, t5217: F, t5279: F, t5391: F, t600: F, t6853: F, t6875: F, t6880: F, t6896: F, t6939: F, t6961: F) -> (F, F, F) {
    let t20085 = t5257 * t6958;
    let t20093 = t1034 * t5367 * t164;
    let t20102 = t2639 * t1753 * t164;
    let t20106 = 0.25724410870841842183e-2 * t1733 * t179 * t6880 * t6939 + 0.12004725073059526352e0 * t20037 - 0.12862205435420921092e-1 * t5279 * t179 * t6875 * t6939 + 0.25724410870841842183e-2 * t1733 * t179 * t6961 * t1734 + 0.25724410870841842183e-2 * t1733 * t179 * t2575 * t1753 * t164 + 0.85748036236139473944e-3 * t1733 * t179 * t1020 * t5367 * t164 + 0.30011812682648815881e-2 * t20057 - 0.77173232612525526549e-1 * t20060 * t179 * t19934 + 0.51448821741683684368e-2 * t20065 * t179 * t20067 - 0.77173232612525526552e-2 * t6896 * t179 * t20071 + 0.51448821741683684367e-2 * t17067 * t179 * t20075 * t5391 - 0.64311027177104605458e-3 * t2645 * t179 * t20081 - 0.24009450146119052704e-1 * t20085 + 0.25724410870841842183e-2 * t1733 * t179 * t6853 * t600 * t164 - 0.21437009059034868486e-3 * t2645 * t179 * t20093 + 0.85748036236139473944e-3 * t1733 * t179 * t2600 * t5217 - 0.64311027177104605458e-3 * t2645 * t179 * t20102;
    (t20093, t20102, t20106)
}
