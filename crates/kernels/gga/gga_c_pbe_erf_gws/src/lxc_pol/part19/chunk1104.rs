//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1104/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1104<F: Float>(t52901: F, t52930: F, t52961: F, t52968: F, t14311: F, t3083: F, t4083: F, t8669: F, t4110: F, t8589: F, t829: F, t830: F, t52991: F, t53011: F, t14182: F, t26958: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t54886 = 7.0 / 576.0 * t52901;
    let t54896 = 7.0 / 72.0 * t52930;
    let t54902 = 7.0 / 1152.0 * t52961;
    let t54904 = 7.0 / 576.0 * t52968;
    let t54911 = 7.0 / 144.0 * t3083 * t14311;
    let t54915 = 7.0 / 144.0 * t8669 * t4083;
    let t54916 = t8589 * t4110;
    let t54918 = t829 * t830 * t54916;
    let t54923 = 7.0 / 72.0 * t52991;
    let t54927 = 7.0 / 1152.0 * t53011;
    let t54937 = 7.0 / 72.0 * t26958 * t14182;
    (t54886, t54896, t54902, t54904, t54911, t54915, t54918, t54923, t54927, t54937)
}
