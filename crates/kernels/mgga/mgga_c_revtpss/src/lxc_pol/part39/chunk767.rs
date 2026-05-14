//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 767/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk767<F: Float>(t1370: F, t1388: F, t3926: F, t3931: F, t3934: F, t3940: F, t3944: F, t3946: F, t3950: F, t3953: F, t3956: F, t3958: F, t3961: F, t3967: F, t4065: F) -> (F,) {
    let t4066 = -0.21437009059034868486e-3 * t1388 * t3926 + 0.20007875121765877254e-2 * t3931 + 0.17149607247227894789e-2 * t3934 * t3940 + t3944 * t3946 / 16.0 + t3950 + 0.57165357490759649296e-4 * t3953 + t3956 + 7.0 / 72.0 * t3958 - t1370 * t3961 / 48.0 + t3967 + t4065;
    (t4066,)
}
