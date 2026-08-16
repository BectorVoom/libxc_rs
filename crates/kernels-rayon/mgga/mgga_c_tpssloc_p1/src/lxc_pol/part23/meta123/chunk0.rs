//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 618/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk618(t17: f64, t5168: f64, t1408: f64, t3704: f64, t1649: f64, t3711: f64, t1804: f64, t3726: f64, t131: f64, t3732: f64, t205: f64, t1799: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5169 = t17 * t5168;
    let t5170 = t3704 * t1408;
    let t5178 = t3711 * t1649;
    let t5192 = t3726 * t1804;
    let t5194 = t3732 * t131;
    let t5195 = t205 * t5194;
    let t5196 = t213 * t1799;
    (t5169, t5170, t5178, t5192, t5194, t5195, t5196)
}
