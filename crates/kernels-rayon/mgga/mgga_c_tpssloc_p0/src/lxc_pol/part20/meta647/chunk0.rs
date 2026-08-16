//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2375/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375(t10662: f64, t14395: f64, t42028: f64, t10829: f64, t14258: f64, t959: f64, t10605: f64, t4483: f64, t10523: f64, t2933: f64, t4471: f64, t47793: f64, t47795: f64, t47798: f64, t47802: f64, t48679: f64, t48681: f64, t48725: f64, t48727: f64, t48730: f64, t48732: f64, t48734: f64, t48736: f64, t48738: f64, t48741: f64, t48744: f64, t48747: f64) -> (f64, f64, f64, f64, f64) {
    let t48750 = 0.62071215503128080361e4_f64 * t42028 * t14395 * t10662;
    let t48753 = 0.6233709278045326953e3_f64 * t959 * t14258 * t10829;
    let t48755 = 0.5848223622634646207e0_f64 * t4483 * t10605;
    let t48759 = 0.31168546390226634765e3_f64 * t959 * t10523 * t4471 * t2933;
    let t48760 = t47793 - t47795 + t47798 + t47802 - t48679 - t48681 - t48725 - t48727 - t48730 - t48732 - t48734 + t48736 + t48738 - t48741 - t48744 - t48747 - t48750 - t48753 - t48755 + t48759;
    (t48750, t48753, t48755, t48759, t48760)
}
