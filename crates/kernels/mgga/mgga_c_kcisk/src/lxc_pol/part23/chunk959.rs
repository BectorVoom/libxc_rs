//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 959/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk959<F: Float>(t5741: F, t827: F, t5738: F, t19123: F, t3661: F, t26: F, t19114: F, t1186: F, t19127: F, t12941: F, t19109: F, t398: F, t442: F, t311: F, t3841: F, t19119: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19483 = t827 * t5741;
    let t19484 = 0.21908444444444444444e0 * t19483;
    let t19485 = t827 * t5738;
    let t19487 = t3661 * t19123;
    let t19488 = t26 * t19487;
    let t19490 = t3661 * t19114;
    let t19491 = t26 * t19490;
    let t19493 = t1186 * t19127;
    let t19494 = t26 * t19493;
    let t19496 = t12941 * t19109;
    let t19497 = t26 * t19496;
    let t19508 = t398 * t442;
    let t19510 = t311 * t3841 * t19508;
    let t19512 = t3661 * t19119;
    (t19483, t19484, t19485, t19488, t19491, t19494, t19497, t19508, t19510, t19512)
}
