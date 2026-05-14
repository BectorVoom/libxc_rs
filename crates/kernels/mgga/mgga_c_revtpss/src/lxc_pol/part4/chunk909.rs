//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 909/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk909<F: Float>(t1413: F, t3889: F, t547: F, t807: F, t9646: F, t2236: F, t66: F, t240: F, t550: F, t268: F, t64: F, t8779: F, t159: F, t535: F, t65: F, t235: F) -> (F, F, F, F, F, F, F, F) {
    let t9714 = t1413 * t3889;
    let t9715 = t547 * t9714;
    let t9716 = t807 * t9715;
    let t9718 = t9646 * t547;
    let t9720 = 1.0 / t66 / t2236;
    let t9721 = t9720 * t240;
    let t9722 = t9721 * t550;
    let t9723 = t9722 * t268;
    let t9725 = 0.20082057720118594944e-6 * t9718 * t9723;
    let t9726 = t64 * t8779;
    let t9727 = t9726 * t159;
    let t9729 = 455.0 / 1296.0 * t9727 * t535;
    let t9731 = 1.0 / t65 / t2236;
    let t9732 = t235 * t9731;
    (t9716, t9720, t9721, t9725, t9727, t9729, t9731, t9732)
}
