//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1012/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1012<F: Float>(t1017: F, t5968: F, t574: F, t605: F, t23443: F, t3446: F, t23470: F, t3430: F, t3435: F, t1378: F, t2097: F) -> (F, F, F, F, F, F) {
    let t26849 = t5968 * t1017;
    let t26851 = t574 * t605 * t26849;
    let t26854 = t23443 * t3446;
    let t26857 = t23470 * t3430;
    let t26860 = t23470 * t3435;
    let t26863 = t2097 * t1378;
    (t26849, t26851, t26854, t26857, t26860, t26863)
}
