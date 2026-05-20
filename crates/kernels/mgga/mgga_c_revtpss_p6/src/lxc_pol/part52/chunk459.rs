//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 459/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk459<F: Float>(t2689: F, t810: F, t775: F, t854: F, t236: F, t807: F, t21: F, t65: F, t64: F, t159: F, t222: F, t794: F, t798: F) -> (F, F, F, F, F, F, F) {
    let t2691 = F::cast_from(0.76220476654346199061e-4_f64) * t2689 * t810;
    let t2693 = t854 * t775;
    let t2694 = t236 * t2693;
    let t2695 = t807 * t2694;
    let t2698 = F::new(1.0) / t65 / t21;
    let t2699 = t64 * t2698;
    let t2700 = t2699 * t159;
    let t2702 = F::new(35.0) / F::new(432.0) * t2700 * t222;
    let t2703 = t794 * t798;
    (t2691, t2693, t2695, t2698, t2700, t2702, t2703)
}
