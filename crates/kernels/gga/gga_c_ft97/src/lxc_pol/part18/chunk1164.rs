//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1164/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1164<F: Float>(t11437: F, t5570: F, t8101: F, t1608: F, t92439: F, t1630: F, t5546: F, t45488: F, t930: F, t1656: F, t25778: F, t100483: F, t100491: F, t100496: F, t100504: F, t100508: F, t100512: F, t100521: F, t100524: F, t100526: F, t1302: F, t22514: F, t22534: F, t22603: F, t22820: F, t25708: F, t44969: F, t53: F, t5540: F, t5611: F, t6426: F, t6427: F, t72: F, t73: F, t92264: F, t92275: F, t92299: F, t92354: F, t92371: F, t92379: F, t92380: F, t92386: F, t92389: F, t925: F, t929: F, t93047: F, t93048: F, t93078: F) -> (F, F, F, F, F) {
    let t100530 = t5570 * t8101 * t11437;
    let t100540 = t1608 * t92439;
    let t100541 = t5546 * t1630;
    let t100542 = t930 * t45488;
    let t100546 = t25778 * t1656;
    let t100550 = -0.61394644015449158009e-7 * t100483 * t92354 * t22514 * t72 * t929 * t53 + 0.68099848938271604939e-1 * t5611 * t100491 + 0.2979368391049382716e-1 * t100496 + 0.12112685275721489029e-7 * t92379 * t1302 * t6426 * t92380 + 0.60548059007656442388e-3 * t92264 + 0.51074886703703703704e-1 * t92275 + 0.89019191601965515283e-5 * t22534 * t73 * t100504 - 0.14836531933660919214e-4 * t22534 * t73 * t100508 + 0.24710505058474293383e-6 * t93078 * t73 * t100512 - 0.3268136001329198891e-5 * t92299 * t6427 * t44969 - t100521 - t100524 + 0.12768721675925925926e-1 * t25708 * t100526 + 0.51074886703703703704e-1 * t25708 * t100530 + 0.60548059007656442388e-3 * t93047 * t93048 * t925 * t22820 - 0.17263005832038132093e-5 * t92371 - 0.21281202793209876543e-2 * t92386 - 0.28374937057613168724e-2 * t92389 + 0.10338048737805743098e-4 * t100540 * t100541 * t100542 - 0.25845121844514357744e-4 * t22603 * t5540 * t100546;
    (t100530, t100541, t100542, t100546, t100550)
}
