//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1098/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1098<F: Float>(t13972: F, t14721: F, t13808: F, t14776: F, t2306: F, t3037: F, t3074: F, t331: F, t833: F, t14469: F, t50884: F, t13798: F, t3972: F, t50956: F, t8827: F, t13776: F, t3212: F, t875: F) -> (F, F, F, F, F, F) {
    let t53727 = t13972 * t14721;
    let t53729 = t13808 * t14776;
    let t53734 = t3074 * t2306 * t3037 * t331 * t833;
    let t53736 = t50884 * t14469;
    let t53742 = t3972 * t50956 * t8827 * t13798;
    let t53748 = t13776 * t50956 * t3212 * t875;
    (t53727, t53729, t53734, t53736, t53742, t53748)
}
