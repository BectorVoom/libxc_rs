//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 645/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk645<F: Float>(t3484: F, t5635: F, t5633: F, t2181: F, t443: F, t1056: F, t1354: F, t2059: F, t1364: F, t220: F, t425: F, t1346: F, t2192: F, t2191: F, t3831: F, t1175: F, t2083: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5636 = t3484 * t5635;
    let t5637 = t5633 * t5636;
    let t5641 = t443 * t2181;
    let t5643 = t2181 * t1056;
    let t5646 = t1354 * t2059;
    let t5647 = t5646 * t1364;
    let t5650 = t425 * t220;
    let t5653 = t1346 * t2192;
    let t5655 = t2192 * t1056;
    let t5658 = t3831 * t2191;
    let t5659 = t5658 * t1364;
    let t5662 = t2083 * t1175;
    (t5636, t5637, t5641, t5643, t5646, t5647, t5650, t5653, t5655, t5658, t5659, t5662)
}
