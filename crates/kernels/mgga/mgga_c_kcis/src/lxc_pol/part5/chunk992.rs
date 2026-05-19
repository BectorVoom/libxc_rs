//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 992/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk992<F: Float>(t1507: F, t456: F, t1444: F, t1455: F, t1523: F, t318: F, t86: F, t334: F, t565: F, t3754: F, t1520: F, t752: F) -> (F, F, F, F, F, F) {
    let t12361 = t1507 * t456;
    let t12371 = t1455 * t1444;
    let t12397 = t86 * t318 * t1523;
    let t12401 = F::cast_from(0.11791604938271604938e-1_f64) * t86 * t334 * t565;
    let t12406 = t1455 * t3754;
    let t12431 = t752 * t1520;
    (t12361, t12371, t12397, t12401, t12406, t12431)
}
