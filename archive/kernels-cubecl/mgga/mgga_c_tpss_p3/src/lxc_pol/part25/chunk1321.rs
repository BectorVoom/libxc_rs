//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1321/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1321<F: Float>(t14185: F, t17964: F, t14304: F, t5547: F, t14229: F, t14234: F, t14176: F, t19703: F, t4708: F, t61072: F, t17946: F, t4712: F) -> (F, F, F, F, F, F, F) {
    let t69968 = t17964 * t14185;
    let t69972 = t5547 * t14304;
    let t69974 = t17964 * t14229;
    let t69976 = t17964 * t14234;
    let t69978 = t19703 * t14176;
    let t69981 = t61072 * t4708;
    let t69983 = t17946 * t4712;
    (t69968, t69972, t69974, t69976, t69978, t69981, t69983)
}
