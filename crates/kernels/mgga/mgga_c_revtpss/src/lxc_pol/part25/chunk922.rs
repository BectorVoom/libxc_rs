//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 922/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk922<F: Float>(t10652: F, t231: F, t2783: F, t2782: F, t10069: F, t2786: F, t10073: F, t836: F, t860: F, t251: F, t2645: F, t10111: F, t22: F, t870: F) -> (F, F, F, F, F, F, F) {
    let t10920 = t2783 * t10652 * t231;
    let t10921 = t2782 * t10920;
    let t10923 = t10069 * t2786;
    let t10925 = t10073 * t2786;
    let t10929 = t2783 * t860 * t836 * t231;
    let t10930 = t2782 * t10929;
    let t10932 = t251 * t2645;
    let t10934 = t2783 * t10932 * t231;
    let t10935 = t2782 * t10934;
    let t10939 = F::new(0.19637199382202157274e-3) * t10111 * t870 * t22;
    (t10921, t10923, t10925, t10930, t10932, t10935, t10939)
}
