//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 740/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk740<F: Float>(t12680: F, t2213: F, t1017: F, t2230: F, t574: F, t1060: F, t2075: F, t12561: F, t167: F, t3408: F, t616: F, t9132: F) -> (F, F, F, F, F, F) {
    let t12681 = t12680 * t2213;
    let t12685 = t574 * t2230 * t1017;
    let t12689 = t574 * t1060 * t2075;
    let t12696 = t574 * t167 * t12561;
    let t12700 = t574 * t616 * t3408;
    let t12703 = t9132 * t167;
    (t12681, t12685, t12689, t12696, t12700, t12703)
}
