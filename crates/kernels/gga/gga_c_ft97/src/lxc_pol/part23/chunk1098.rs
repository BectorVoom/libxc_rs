//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1098/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1098<F: Float>(t1439: F, t2999: F, t89: F, t6094: F, t8232: F, t2567: F, t6148: F, t96953: F, t96958: F, t96983: F, t97029: F, t97061: F, t97123: F, t97232: F, t97244: F, t6085: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t97247 = t89 * t2999 * t1439;
    let t97248 = 28.0 / 27.0 * t97247;
    let t97269 = t8232 * t6094;
    let t97299 = t6148 * t2567;
    let t97328 = 4.0 / 27.0 * t96953;
    let t97330 = 2.0 / 9.0 * t96958;
    let t97338 = 2.0 / 27.0 * t96983;
    let t97352 = 14.0 / 81.0 * t97029;
    let t97360 = 8.0 / 9.0 * t97061;
    let t97377 = 4.0 / 9.0 * t97123;
    let t97407 = t97232 / 9.0;
    let t97411 = 4.0 / 9.0 * t97244;
    let t97412 = 28.0 / 81.0 * t97247;
    let t97470 = t8232 * t6085;
    (t97248, t97269, t97299, t97328, t97330, t97338, t97352, t97360, t97377, t97407, t97411, t97412, t97470)
}
