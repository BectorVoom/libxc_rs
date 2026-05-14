//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 975/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk975<F: Float>(t22663: F, t7055: F, t1814: F, t8500: F, t1648: F, t4629: F, t1824: F, t8501: F, t22387: F, t4726: F, t26: F, t1659: F, t22592: F, t827: F, t8564: F, t22564: F, t22567: F, t22570: F, t22573: F, t22578: F, t22581: F, t22586: F, t22589: F, t22594: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22664 = t7055 * t22663;
    let t22667 = t1814 * t8500;
    let t22668 = t22667 * t1648;
    let t22669 = t4629 * t22668;
    let t22672 = t8501 * t1824;
    let t22673 = t7055 * t22672;
    let t22683 = t4726 * t22387;
    let t22684 = t26 * t22683;
    let t22694 = t1659 * t22592;
    let t22695 = t26 * t22694;
    let t22698 = t827 * t8564;
    let t22701 = -0.33547222222222222222e0 * t22567 + 0.12077e1 * t22570 + 0.80513333333333333332e0 * t22573 - 0.181155e1 * t22578 - 0.24154e1 * t22581 - 0.20128333333333333333e0 * t22586 + 0.60385e0 * t22589 - 0.82785e-1 * t22695 - 0.301925e0 * t22594 + 0.18396666666666666667e-1 * t22698 + 0.67094444444444444443e-1 * t22564;
    (t22664, t22668, t22669, t22672, t22673, t22684, t22695, t22698, t22701)
}
