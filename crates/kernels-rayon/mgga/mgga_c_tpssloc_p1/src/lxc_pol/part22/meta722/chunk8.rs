//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2364/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2364(t2: f64, t5669: f64, t584: f64, t21589: f64, t2940: f64, t21152: f64, t690: f64) -> (f64, f64, f64) {
    let t68427 = 3.0_f64 * t5669 * t2 * t584;
    let t68441 = 0.5848223622634646207e0_f64 * t2940 * t21589;
    let t68442 = t690 * t21152;
    (t68427, t68441, t68442)
}
