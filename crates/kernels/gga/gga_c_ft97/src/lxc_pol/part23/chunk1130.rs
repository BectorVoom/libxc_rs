//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1130/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1130<F: Float>(t24361: F, t24378: F, t27638: F, t1095: F, t2394: F, t2379: F, t6776: F, t695: F, t3758: F, t108685: F, t6056: F, t6055: F, t27511: F, t3626: F, t6044: F, t6832: F, t96535: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t109069 = t24361 * t24378 * t27638;
    let t109080 = t2394 * t1095;
    let t109084 = t2379 * t1095;
    let t109108 = t695 * t6776;
    let t109109 = t3758 * t109108;
    let t109117 = t108685 * t6056;
    let t109119 = 0.1134997482304526749e-1 * t6055 * t109117;
    let t109124 = t6044 * t3626 * t27511;
    let t109125 = t6055 * t109124;
    let t109127 = t96535 * t6832;
    (t109069, t109080, t109084, t109108, t109109, t109117, t109119, t109124, t109125, t109127)
}
