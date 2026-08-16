//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 721/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk721(t425: f64, t7614: f64, t431: f64, t7546: f64, t7550: f64, t7551: f64, t7558: f64, t7562: f64, t7567: f64, t7572: f64, t7574: f64, t7578: f64, t7581: f64, t7590: f64, t7593: f64, t7597: f64, t7602: f64, t7603: f64, t7607: f64, t7608: f64, t7612: f64) -> (f64, f64, f64) {
    let t7615 = t7614 * t425;
    let t7616 = 0.16006300097412701803e-1_f64 * t7615;
    let t7617 = t7614 * t431;
    let t7619 = t7546 + t7550 - 0.94344276868812456204e-2_f64 * t7551 - t7558 + 0.34299214494455789578e-2_f64 * t7562 + 0.18868855373762491241e-2_f64 * t7567 + t7572 + t7574 + 0.21437009059034868486e-2_f64 * t7578 - 0.21437009059034868486e-3_f64 * t7581 - t7590 - t7593 / 384.0_f64 - 0.38203125e-2_f64 * t7597 - t7602 + 0.85748036236139473944e-3_f64 * t7603 - t7607 - 0.85748036236139473944e-3_f64 * t7608 + t7612 + t7616 - 0.80031500487063509015e-2_f64 * t7617;
    (t7615, t7617, t7619)
}
