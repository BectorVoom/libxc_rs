//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 799/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk799<F: Float>(t2779: F, t4673: F, t188: F, t7937: F, t1457: F, t7963: F, t2788: F, t4614: F, t2846: F, t1392: F, t2778: F, t1391: F, t2787: F, t600: F, t7861: F, t568: F) -> (F, F, F, F, F, F, F, F) {
    let t8347 = t4673 * t2779;
    let t8352 = t188 * t7937;
    let t8355 = t1457 * t7963;
    let t8358 = t4614 * t2788;
    let t8361 = t4614 * t2846;
    let t8366 = t1392 * t2778;
    let t8367 = t1391 * t8366;
    let t8370 = t1392 * t2787;
    let t8371 = t1391 * t8370;
    let t8380 = t600 * t7861;
    let t8381 = t568 * t8380;
    (t8347, t8352, t8355, t8358, t8361, t8367, t8371, t8381)
}
