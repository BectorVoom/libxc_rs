//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 458/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk458<F: Float>(t2689: F, t810: F, t21: F, t65: F, t64: F, t159: F, t222: F, t794: F, t798: F, t234: F, t2453: F) -> (F, F, F, F, F, F, F) {
    let t2691 = F::cast_from(0.76220476654346199061e-4_f64) * t2689 * t810;
    let t2698 = F::cast_from(1.0_f64) / t65 / t21;
    let t2699 = t64 * t2698;
    let t2700 = t2699 * t159;
    let t2702 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2700 * t222;
    let t2703 = t794 * t798;
    let t2710 = t2453 * t234;
    (t2691, t2698, t2699, t2700, t2702, t2703, t2710)
}
