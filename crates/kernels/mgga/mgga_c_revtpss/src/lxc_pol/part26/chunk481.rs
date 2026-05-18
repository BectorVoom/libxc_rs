//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 481/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk481<F: Float>(t251: F, t836: F, t231: F, t2783: F, t2782: F, t233: F, t860: F, t869: F, t689: F, t136: F, t2457: F, t2710: F) -> (F, F, F, F, F, F, F, F) {
    let t2784 = t251 * t836;
    let t2786 = t2783 * t2784 * t231;
    let t2787 = t2782 * t2786;
    let t2789 = t233 * t860;
    let t2790 = t869 * t2789;
    let t2791 = t689 * t2790;
    let t2793 = t251 * t136;
    let t2796 = F::new(0.11565819519348392139e-2) * t2710 * t2793 * t2457;
    (t2784, t2786, t2787, t2789, t2790, t2791, t2793, t2796)
}
