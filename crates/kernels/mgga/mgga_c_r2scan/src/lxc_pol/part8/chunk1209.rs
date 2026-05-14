//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1209/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1209<F: Float>(t24994: F, t6214: F, t20407: F, t5146: F, t2545: F, t494: F, t6194: F, t410: F, t7124: F, t4911: F, t899: F, t4982: F, t7030: F, t2788: F, t4973: F, t5100: F, t7407: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24995 = t24994 * t6214;
    let t24996 = 0.19043987679069580388e-1 * t24995;
    let t24997 = t5146 * t20407;
    let t24999 = t2545 * t494 * t6194;
    let t25000 = t24997 * t24999;
    let t25001 = 0.48787202696913915093e-3 * t25000;
    let t25031 = t410 * t7124;
    let t25032 = 24.0 * t25031;
    let t25038 = t4911 * t899;
    let t25040 = t4982 * t899;
    let t25041 = 144.0 * t25040;
    let t25042 = t410 * t7030;
    let t25044 = t2788 * t4973;
    let t25181 = t5100 * t7407;
    (t24996, t24997, t24999, t25001, t25032, t25038, t25041, t25042, t25044, t25181)
}
