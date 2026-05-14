//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 811/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk811<F: Float>(t13086: F, t64: F, t10657: F, t871: F, t2919: F, t3113: F, t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t43069: F, t739: F, t2508: F, t2717: F, t3433: F) -> (F, F, F) {
    let t43071 = 4.0 / 3.0 * t13086 * t64;
    let t43072 = t10657 * t871;
    let t43073 = t2919 * t3113;
    let t43075 = 7.0 / 512.0 * t40612;
    let t43076 = 63.0 / 16384.0 * t40614;
    let t43077 = 63.0 / 1048576.0 * t40620;
    let t43078 = 21.0 / 1048576.0 * t40630;
    let t43079 = 21.0 / 16384.0 * t40632;
    let t43080 = 7.0 / 1536.0 * t40634;
    let t43081 = t43069 - t43071 + t43072 - t43073 / 2.0 + t43075 + t43076 - t43077 + t43078 - t43079 - t43080;
    let t43082 = t739 * t43081;
    let t43087 = t2508 * t2717 * t3433;
    (t43081, t43082, t43087)
}
