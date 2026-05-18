//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 985/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk985<F: Float>(t3754: F, t89: F, t4034: F, t516: F, t1444: F, t160: F, t833: F, t1441: F, t450: F, t1431: F, t2466: F, t1438: F, t2471: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11920 = t89 * t3754;
    let t11939 = F::new(1.0) / t4034 / t516;
    let t11951 = t160 * t1444;
    let t11952 = t11951 * t833;
    let t11960 = t1441 * t833;
    let t11966 = t160 * t450;
    let t11967 = F::new(0.71734315950379065738e-1) * t11966;
    let t12003 = t2466 * t1431;
    let t12005 = t2471 * t1438;
    (t11920, t11939, t11951, t11952, t11960, t11966, t11967, t12003, t12005)
}
