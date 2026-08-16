//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 810/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk810<F: Float>(t1227: F, t937: F, t2363: F, t3199: F, t410: F, t3258: F, t6523: F, t1245: F, t3246: F, t914: F, t2393: F, t3308: F, t452: F) -> (F, F, F, F, F, F, F) {
    let t8511 = t937 * t1227;
    let t8512 = t2363 * t8511;
    let t8515 = t410 * t3199;
    let t8516 = t2363 * t8515;
    let t8519 = t6523 * t3258;
    let t8546 = t2363 * t1245;
    let t8549 = t914 * t3246;
    let t8554 = t2393 * t1245;
    let t8599 = t3308 * t452;
    (t8512, t8516, t8519, t8546, t8549, t8554, t8599)
}
