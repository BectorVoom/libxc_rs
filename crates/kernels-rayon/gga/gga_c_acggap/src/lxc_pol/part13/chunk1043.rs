//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1043/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1043(t1078: f64, t2030: f64, t2313: f64, t361: f64, t8816: f64, t30330: f64, t30334: f64, t1181: f64, t5087: f64, t604: f64, t7426: f64, t30343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34327 = t2030 * t1078 * t2313;
    let t34330 = t2030 * t361 * t8816;
    let t34332 = 0.21437009059034868486e-2_f64 * t30330;
    let t34333 = 0.85748036236139473944e-3_f64 * t30334;
    let t34336 = t7426 * t1181 * t604 * t5087;
    let t34338 = 0.10718504529517434243e-2_f64 * t30343;
    (t34327, t34330, t34332, t34333, t34336, t34338)
}
