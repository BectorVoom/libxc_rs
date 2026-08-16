//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1326/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1326(t32: f64, t607: f64, t33114: f64, t645: f64, t8513: f64, t7440: f64, t79: f64, t641: f64, t33118: f64, t6504: f64, t26043: f64, t8307: f64) -> (f64, f64, f64, f64, f64) {
    let t119931 = t32 * t607;
    let t119938 = t8513 * t33114 * t645;
    let t119942 = t79 * t7440;
    let t119944 = t8513 * t119942 * t641;
    let t119952 = t8513 * t33118 * t6504;
    let t119965 = t8513 * t8307 * t26043;
    (t119931, t119938, t119944, t119952, t119965)
}
