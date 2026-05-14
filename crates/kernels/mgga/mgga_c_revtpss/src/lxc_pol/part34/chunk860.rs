//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 860/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk860<F: Float>(t22718: F, t38: F, t10389: F, t10398: F, t22671: F, t22688: F, t4227: F, t4232: F, t5825: F, t633: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t21686: F, t22662: F, t22665: F, t22673: F, t22676: F, t22681: F, t5820: F, t5827: F, t5830: F, t5855: F, t5869: F, t71: F, t85: F) -> (F, F) {
    let t22719 = t38 * t22718;
    let t22738 = -280.0 / 27.0 * t10389 * t22688 + 28.0 / 3.0 * t4227 * t5825 - 4.0 / 3.0 * t633 * t22671 + 280.0 / 27.0 * t10398 * t22688 + 28.0 / 3.0 * t4232 * t5825 + 4.0 / 3.0 * t637 * t22671;
    let t22739 = t77 * t22738;
    let t22742 = -t21686 * t22662 / 4.0 - t22665 * t85 / 4.0 - t5820 * t1494 / 4.0 - t22673 * t85 / 12.0 - t22676 * t85 / 4.0 - t5827 * t1494 / 4.0 - t22681 * t85 / 4.0 - t5830 * t1494 / 2.0 - t1471 * t5869 / 4.0 + t22719 * t85 / 24.0 + t5855 * t1494 / 8.0 + t1487 * t5869 / 8.0 + t71 * t22739 / 24.0;
    (t22738, t22742)
}
