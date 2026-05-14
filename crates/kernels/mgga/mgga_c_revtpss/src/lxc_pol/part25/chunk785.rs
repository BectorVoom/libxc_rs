//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 785/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk785<F: Float>(t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F, t1413: F, t3889: F, t547: F, t807: F, t9646: F, t2236: F, t66: F, t268: F, t64: F, t8779: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9707 = t2237 * t240;
    let t9709 = t9707 * t550 * t816;
    let t9711 = 0.12846167376791569079e-2 * t1379 * t9709;
    let t9712 = t2689 * t3952;
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
    (t9707, t9709, t9711, t9712, t9714, t9716, t9720, t9721, t9723, t9725, t9726)
}
