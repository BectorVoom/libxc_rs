//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1051/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1051<F: Float>(t10727: F, t2723: F, t10726: F, t2661: F, t231: F, t2662: F, t221: F, t2430: F, t2675: F, t2674: F, t2735: F, t2783: F) -> (F, F, F, F) {
    let t10728 = t10727 * t2723;
    let t10729 = t10726 * t10728;
    let t10730 = t2661 * t10729;
    let t10732 = t10727 * t231;
    let t10733 = t2662 * t10732;
    let t10734 = t2661 * t10733;
    let t10741 = t2675 * t221 * t2430;
    let t10742 = t2674 * t10741;
    let t10744 = t2735 * t2783;
    (t10730, t10734, t10742, t10744)
}
