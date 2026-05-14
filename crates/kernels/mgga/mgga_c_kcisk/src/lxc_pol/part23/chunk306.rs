//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 306/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk306<F: Float>(t1161: F, t1165: F, t1176: F, t1175: F, t1355: F, t306: F) -> (F, F, F) {
    let t1359 = 0.41275e-2 * t1161;
    let t1361 = 0.1982e-1 * t1176 - t1359 - 0.41275e-2 * t1165;
    let t1364 = t1355 * t1175 / 4.0 + t306 * t1361 / 2.0;
    (t1359, t1361, t1364)
}
