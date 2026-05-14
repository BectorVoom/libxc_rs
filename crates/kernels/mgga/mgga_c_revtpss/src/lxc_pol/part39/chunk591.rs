//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 591/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk591<F: Float>(t2754: F, t827: F, t828: F, t2695: F, t2702: F, t2704: F, t2707: F, t2716: F, t2721: F, t2726: F, t2730: F, t2732: F, t2739: F, t2742: F, t2745: F, t2751: F, t799: F, t825: F) -> (F, F) {
    let t2756 = t827 * t828 * t2754;
    let t2759 = 0.57165357490759649296e-4 * t2695 + t2702 + 7.0 / 72.0 * t2704 - t799 * t2707 / 48.0 + t2716 + 0.42874018118069736972e-3 * t2721 * t2726 + t2730 * t2732 / 16.0 - t2739 + 0.20007875121765877254e-2 * t2742 + 0.17149607247227894789e-2 * t2745 * t2751 - 0.21437009059034868486e-3 * t825 * t2756;
    (t2756, t2759)
}
