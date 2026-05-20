//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1118/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1118<F: Float>(t11687: F, t4894: F, t3117: F, t4900: F, t2258: F, t3094: F, t3093: F, t3092: F, t11644: F, t11649: F, t11653: F, t11656: F, t11663: F, t11667: F, t11672: F, t11675: F, t11680: F, t11684: F, t3091: F, t3097: F, t3130: F, t3136: F, t3169: F, t4837: F, t4892: F, t4899: F) -> (F, F, F, F, F, F, F, F) {
    let t11688 = t11687 * t4894;
    let t11689 = t3117 * t11688;
    let t11692 = t11687 * t4900;
    let t11693 = t3117 * t11692;
    let t11696 = t3094 * t2258;
    let t11697 = t3093 * t11696;
    let t11698 = t3092 * t11697;
    let t11701 = -F::cast_from(0.57165357490759649295e-3_f64) * t11644 - F::cast_from(0.34299214494455789577e-2_f64) * t3169 * t3136 + F::cast_from(0.42874018118069736972e-3_f64) * t11649 + F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t11653 + F::cast_from(0.45732285992607719436e-2_f64) * t11656 * t3130 + F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t11663 - F::cast_from(0.42874018118069736972e-3_f64) * t4899 * t11667 - F::cast_from(0.45732285992607719436e-2_f64) * t11672 * t3097 + F::cast_from(0.85748036236139473944e-3_f64) * t11675 * t3097 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t11680 - F::cast_from(0.85748036236139473944e-3_f64) * t3091 * t11684 + F::cast_from(0.12862205435420921092e-2_f64) * t4892 * t11689 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t11693 + F::cast_from(0.42874018118069736972e-3_f64) * t3091 * t11698;
    (t11688, t11689, t11692, t11693, t11696, t11697, t11698, t11701)
}
