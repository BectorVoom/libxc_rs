//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1791/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1791(t2: f64, t873: f64, t584: f64, t265: f64, t16: f64, t4331: f64, t10723: f64, t4496: f64, t959: f64, t2944: f64, t4483: f64, t2940: f64, t4493: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13501 = t873 * t2;
    let t13503 = 2.0_f64 * t13501 * t584;
    let t13504 = t265 * t584;
    let t13506 = 3.0_f64 * t4331 * t16;
    let t13508 = t4496 * t10723;
    let t13510 = 0.17315859105681463759e2_f64 * t959 * t13508;
    let t13512 = 0.11696447245269292414e1_f64 * t4483 * t2944;
    let t13514 = 0.11696447245269292414e1_f64 * t2940 * t4493;
    (t13503, t13504, t13506, t13508, t13510, t13512, t13514)
}
