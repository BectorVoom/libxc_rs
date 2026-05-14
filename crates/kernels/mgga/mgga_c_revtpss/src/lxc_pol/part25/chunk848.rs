//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 848/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk848<F: Float>(t10760: F, t10761: F, t125: F, t2430: F, t2747: F, t837: F, t2475: F, t72: F, t245: F, t2394: F, t2482: F, t814: F, t823: F, t136: F, t853: F, t220: F) -> (F, F, F, F, F) {
    let t10762 = t10760 * t10761;
    let t10764 = t125 * t2430;
    let t10766 = t2747 * t10764 * t837;
    let t10769 = t2475 * t72;
    let t10770 = t10769 * t245;
    let t10771 = t125 * t2394;
    let t10773 = t10770 * t10771 * t837;
    let t10777 = t2482 * t823 * t814;
    let t10778 = t853 * t136;
    let t10779 = t10778 * t220;
    (t10762, t10766, t10773, t10777, t10779)
}
