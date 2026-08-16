//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 558/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk558(t35: f64, t4466: f64, t374: f64, t1594: f64, t4449: f64, t938: f64, t1711: f64, t64: f64, t1737: f64, t4417: f64, t420: f64, t419: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4467 = t4466 * t35;
    let t4468 = t374 * t4467;
    let t4471 = t1594 * t4449;
    let t4474 = t938 * t938;
    let t4475 = t1711 * t4474;
    let t4476 = t64 * t4475;
    let t4479 = t1737 * t4417;
    let t4480 = t420 * t4479;
    let t4481 = t419 * t4480;
    (t4467, t4468, t4471, t4474, t4476, t4479, t4480, t4481)
}
