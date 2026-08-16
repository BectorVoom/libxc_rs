//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 909/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk909<F: Float>(t140: F, t6658: F, t1222: F, t6662: F, t369: F, t6593: F, t475: F, t467: F, t1256: F, t6602: F, t6595: F, t6598: F) -> (F, F, F, F, F, F) {
    let t21251 = t140 * t6658;
    let t21252 = t1222 * t21251;
    let t21254 = t140 * t6662;
    let t21255 = t1222 * t21254;
    let t21270 = t6593 * t369;
    let t21271 = t475 * t21270;
    let t21272 = t467 * t21271;
    let t21283 = t6602 * t1256;
    let t21285 = t6595 * t1256;
    let t21287 = t6598 * t1256;
    (t21252, t21255, t21272, t21283, t21285, t21287)
}
