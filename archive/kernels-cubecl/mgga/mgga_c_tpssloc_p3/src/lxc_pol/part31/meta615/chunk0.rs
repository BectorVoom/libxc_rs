//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1862/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1862<F: Float>(t17635: F, t605: F, t19334: F, t2235: F, t5392: F, t19534: F, t88: F, t1458: F, t4025: F, t5493: F, t649: F, t5464: F, t81442: F) -> (F, F, F, F, F, F, F) {
    let t96559 = t605 * t17635;
    let t96562 = t605 * t19334;
    let t96646 = t2235 * t5392;
    let t96657 = t88 * t19534;
    let t96683 = t4025 * t1458;
    let t96709 = t649 * t5493;
    let t96713 = t81442 * t5464;
    (t96559, t96562, t96646, t96657, t96683, t96709, t96713)
}
