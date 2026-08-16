//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1324/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1324<F: Float>(t411: F, t6546: F, t3199: F, t937: F, t1245: F, t6514: F, t410: F, t8309: F, t1227: F, t2421: F, t2363: F, t3246: F) -> (F, F, F, F, F, F) {
    let t23398 = t411 * t6546;
    let t23412 = t937 * t3199;
    let t23416 = t6514 * t1245;
    let t23446 = t410 * t8309;
    let t23450 = t2421 * t1227;
    let t23465 = t2363 * t3246;
    (t23398, t23412, t23416, t23446, t23450, t23465)
}
