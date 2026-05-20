//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3022/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3022<F: Float>(t4398: F, t9323: F, t4302: F, t9586: F, t10612: F, t4311: F, t14330: F, t14369: F, t2251: F, t14440: F, t2398: F, t2258: F, t4401: F) -> (F, F, F, F, F, F) {
    let t50852 = t4398 * t9323;
    let t50856 = t4302 * t9586;
    let t50865 = t4311 * t10612;
    let t50868 = t14330 * t14369 * t2251;
    let t50873 = t2398 * t14440;
    let t50878 = t4401 * t14369 * t2258;
    (t50852, t50856, t50865, t50868, t50873, t50878)
}
