//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1341/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1341<F: Float>(t112095: F, t7440: F, t18167: F, t33121: F, t2585: F, t1872: F, t5323: F, t17899: F, t33097: F, t17940: F, t34321: F, t33091: F, t7307: F, t33120: F, t6973: F, t17930: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t117405 = t112095 * t7440;
    let t117407 = t33121 * t18167;
    let t117409 = sigma2 * t2585;
    let t117410 = t1872 * t117409;
    let t117411 = t117410 * t5323;
    let t117413 = t33097 * t17899;
    let t117415 = t34321 * t17940;
    let t117417 = t33091 * t7307;
    let t117419 = t6973 * t33120;
    let t117420 = t117419 * t5323;
    let t117422 = t33121 * t17930;
    (t117405, t117407, t117411, t117413, t117415, t117417, t117420, t117422)
}
