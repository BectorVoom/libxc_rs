//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 985/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk985<F: Float>(t1593: F, t2567: F, t360: F, t2294: F, t2574: F, t2133: F, t133: F, t2526: F, t1605: F, t1604: F, t1610: F, t2201: F, t2687: F, t2719: F, t788: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7462 = t2567 * t1593;
    let t7463 = t360 * t7462;
    let t7466 = t2294 * t2574;
    let t7468 = 0.23115257973478049502e0 * t2133 * t7466;
    let t7469 = t133 * t2526;
    let t7470 = t1605 * t7469;
    let t7472 = 0.10975748638225852664e-1 * t1604 * t7470;
    let t7475 = 0.11643651550782197811e-1 * t2201 * t1610 * t2687;
    let t7476 = t788 * t2719;
    let t7479 = 0.11643651550782197811e-1 * t2201 * t785 * t7476;
    (t7462, t7463, t7466, t7468, t7469, t7470, t7472, t7475, t7476, t7479)
}
