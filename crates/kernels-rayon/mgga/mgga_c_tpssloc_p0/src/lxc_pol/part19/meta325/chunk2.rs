//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1156/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1156(t12313: f64, t3726: f64, t2559: f64, t3732: f64, t3766: f64, t12214: f64, t782: f64, t12320: f64, t154: f64, t1995: f64, t205: f64, t3734: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40012 = t3726 * t12313;
    let t40018 = t2559 * t3732;
    let t40019 = t40018 * t3766;
    let t40021 = t782 * t12214;
    let t40022 = t40021 * t12320;
    let t40024 = t154 * t1995;
    let t40025 = t205 * t40024;
    let t40026 = t3734 * t3734;
    (t40012, t40018, t40019, t40021, t40022, t40025, t40026)
}
