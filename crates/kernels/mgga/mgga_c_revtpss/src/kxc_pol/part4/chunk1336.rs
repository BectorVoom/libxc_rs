//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1336/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1336<F: Float>(t3531: F, t5202: F, t300: F, t5155: F, t1198: F, t3539: F, t5192: F, t12571: F, t1765: F, t16710: F, t16712: F, t12297: F, t12299: F, t12301: F, t12303: F, t12382: F, t16706: F, t16708: F, t16717: F, t16722: F, t16727: F, t16731: F, t16735: F, t16740: F, t16744: F, t16748: F) -> (F, F, F, F, F) {
    let t16783 = F::new(0.11696447245269292414e1) * t3531 * t5202;
    let t16784 = t300 * t5155;
    let t16786 = F::new(0.11696447245269292414e1) * t16784 * t1198;
    let t16788 = F::new(0.5848223622634646207e0) * t5192 * t3539;
    let t16790 = F::new(0.5848223622634646207e0) * t12571 * t1765;
    let t16797 = F::new(0.23744444444444444444e-1) * t16710;
    let t16798 = F::new(0.11872222222222222222e-1) * t16712;
    let t16807 = -t12382 + F::new(0.15829629629629629629e-1) * t12297 + F::new(0.39574074074074074073e-2) * t12299 - F::new(0.11872222222222222222e-1) * t12301 - F::new(0.5936111111111111111e-2) * t12303 + F::new(0.79148148148148148146e-2) * t16706 + F::new(0.79148148148148148146e-2) * t16708 - t16797 - t16798 + F::new(0.19787037037037037037e-1) * t16717 - F::new(0.71233333333333333332e-1) * t16722 - F::new(0.23744444444444444444e-1) * t16727 - F::new(0.11872222222222222222e-1) * t16731 + F::new(0.10685e0) * t16735 + F::new(0.71233333333333333332e-1) * t16740 + F::new(0.35616666666666666666e-1) * t16744 + F::new(0.17808333333333333333e-1) * t16748;
    (t16783, t16786, t16788, t16790, t16807)
}
