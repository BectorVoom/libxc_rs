//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 912/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk912<F: Float>(t1504: F, t561: F, t1507: F, t456: F, t1444: F, t1455: F, t1523: F, t318: F, t86: F, t334: F, t565: F, t3754: F, t1520: F, t752: F, t11824: F, t569: F) -> (F, F, F, F, F, F, F, F) {
    let t12343 = t1504 * t1504;
    let t12344 = 1.0 / t12343;
    let t12345 = t561 * t12344;
    let t12361 = t1507 * t456;
    let t12371 = t1455 * t1444;
    let t12397 = t86 * t318 * t1523;
    let t12401 = 0.11791604938271604938e-1 * t86 * t334 * t565;
    let t12406 = t1455 * t3754;
    let t12431 = t752 * t1520;
    let t12520 = t569 * t11824;
    (t12345, t12361, t12371, t12397, t12401, t12406, t12431, t12520)
}
