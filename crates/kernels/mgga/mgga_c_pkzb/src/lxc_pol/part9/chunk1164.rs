//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1164/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1164<F: Float>(t8235: F, t832: F, t853: F, t2235: F, t8214: F, t2328: F, t8300: F, t2298: F, t8012: F, t898: F, t2317: F, t3161: F, t8098: F, t8013: F, t2336: F, t8028: F) -> (F, F, F, F, F, F, F) {
    let t22357 = t8235 * t832;
    let t22359 = 3.0 * t22357 * t853;
    let t22361 = 3.0 * t8214 * t2235;
    let t22363 = 0.17544670867903938621e1 * t2328 * t8300;
    let t22366 = 0.10526802520742363173e2 * t898 * t8012 * t2298;
    let t22374 = 0.51947577317044391277e2 * t898 * t2317 * t8098 * t3161;
    let t22376 = 0.10389515463408878255e3 * t2328 * t8013;
    let t22378 = 0.17544670867903938621e1 * t8028 * t2336;
    (t22359, t22361, t22363, t22366, t22374, t22376, t22378)
}
