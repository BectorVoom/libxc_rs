//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1963/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1963(t4724: f64, t4899: f64, t1210: f64, t8039: f64, t24721: f64, t1714: f64, t2133: f64, t2132: f64, t6739: f64, t8026: f64, t7325: f64, t25588: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27697 = t4899 * t4724;
    let t27700 = t1210 * t8039;
    let t27701 = t24721 * t27700;
    let t27703 = t2133 * t1714;
    let t27704 = t2132 * t27703;
    let t27710 = t8026 * t6739;
    let t27711 = t27710 * t7325;
    let t27714 = t2132 * t25588;
    (t27697, t27700, t27701, t27704, t27710, t27711, t27714)
}
