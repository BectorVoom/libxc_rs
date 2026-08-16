//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1154/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1154<F: Float>(t1020: F, t164: F, t17067: F, t1733: F, t1734: F, t1753: F, t179: F, t19934: F, t20037: F, t20057: F, t20060: F, t20065: F, t20067: F, t20071: F, t20075: F, t20081: F, t20085: F, t20093: F, t20102: F, t2575: F, t2600: F, t2645: F, t5217: F, t5279: F, t5367: F, t5391: F, t600: F, t6853: F, t6875: F, t6880: F, t6896: F, t6939: F, t6961: F) -> F {
    let t20106 = F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t6880 * t6939 + F::cast_from(0.12004725073059526352e0_f64) * t20037 - F::cast_from(0.12862205435420921092e-1_f64) * t5279 * t179 * t6875 * t6939 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t6961 * t1734 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t2575 * t1753 * t164 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t179 * t1020 * t5367 * t164 + F::cast_from(0.30011812682648815881e-2_f64) * t20057 - F::cast_from(0.77173232612525526549e-1_f64) * t20060 * t179 * t19934 + F::cast_from(0.51448821741683684368e-2_f64) * t20065 * t179 * t20067 - F::cast_from(0.77173232612525526552e-2_f64) * t6896 * t179 * t20071 + F::cast_from(0.51448821741683684367e-2_f64) * t17067 * t179 * t20075 * t5391 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t20081 - F::cast_from(0.24009450146119052704e-1_f64) * t20085 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t6853 * t600 * t164 - F::cast_from(0.21437009059034868486e-3_f64) * t2645 * t179 * t20093 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t179 * t2600 * t5217 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t20102;
    t20106
}
