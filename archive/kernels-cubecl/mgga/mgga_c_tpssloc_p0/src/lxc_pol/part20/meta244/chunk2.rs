//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1355/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1355<F: Float>(t118: F, t168: F, t2477: F, t2510: F, t725: F, t740: F, t9457: F, t9476: F, t9484: F, t9697: F, t9730: F, t9734: F, t9739: F, t9740: F, t9752: F, t9755: F, t9758: F, t9759: F, t9762: F, t9763: F, t9766: F, t9780: F, t9781: F, t9789: F, t9793: F, t9797: F) -> F {
    let t9798 = F::cast_from(0.2069040516770936012e4_f64) * t9730 * t9734 + t9457 - F::cast_from(0.19298375398431042081e3_f64) * t9739 * t9740 + F::cast_from(1.0_f64) * t725 * t9752 + F::cast_from(0.35089341735807877242e1_f64) * t2510 * t9755 - t9476 - t9484 + F::cast_from(0.10254018858216406658e4_f64) * t9758 * t9759 - F::cast_from(0.10389515463408878255e3_f64) * t9762 * t9763 + F::cast_from(0.5848223622634646207e0_f64) * t740 * t9766 - t9780 + F::cast_from(6.0_f64) * t2477 * t9781 + F::cast_from(0.16562821945185185185e-2_f64) * t118 * t9697 * t168 + t9789 - t9793 - t9797;
    t9798
}
