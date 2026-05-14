//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 609/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk609<F: Float>(t289: F, t9343: F, t1614: F, t699: F, t884: F, t2471: F, t333: F, t321: F, t739: F, t2208: F, t4985: F, t1540: F, t708: F, t2463: F, t2604: F, t1632: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9344 = t289 * t9343;
    let t9352 = t699 * t1614;
    let t9353 = t884 * t9352;
    let t9370 = t2471 * t333;
    let t9371 = t884 * t9370;
    let t9383 = t2471 * t321;
    let t9384 = t739 * t9383;
    let t9391 = t4985 * t2208;
    let t9394 = t1540 * t708;
    let t9396 = t2604 * t2463;
    let t9399 = t699 * t1632;
    (t9344, t9352, t9353, t9370, t9371, t9383, t9384, t9391, t9394, t9396, t9399)
}
