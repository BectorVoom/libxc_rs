//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1249/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1249<F: Float>(t17964: F, t4724: F, t4761: F, t5552: F, t4766: F, t4771: F, t5559: F, t4775: F, t1705: F, t4778: F, t935: F, t1378: F, t1395: F, t226: F) -> (F, F, F, F, F, F, F, F) {
    let t21280 = t17964 * t4724;
    let t21282 = t5552 * t4761;
    let t21284 = t5552 * t4766;
    let t21286 = t5559 * t4771;
    let t21288 = t5559 * t4775;
    let t21298 = t1705 * t4778;
    let t21299 = t21298 * t935;
    let t21312 = t1395 * t1378 * t226;
    (t21280, t21282, t21284, t21286, t21288, t21298, t21299, t21312)
}
