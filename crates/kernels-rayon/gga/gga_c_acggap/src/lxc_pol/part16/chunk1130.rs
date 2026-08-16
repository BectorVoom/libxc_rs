//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1130/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1130(t31350: f64, t6343: f64, t30866: f64, t30868: f64, t30872: f64, t30874: f64, t30878: f64, t30880: f64, t30883: f64, t39615: f64, t39617: f64, t39620: f64, t39623: f64, t39626: f64, t39629: f64, t39632: f64, t39640: f64, t39643: f64, t39647: f64) -> f64 {
    let t39649 = t31350 * t6343;
    let t39651 = 7.0_f64 / 288.0_f64 * t39615 - t39617 / 96.0_f64 - t39620 / 64.0_f64 + t39623 / 96.0_f64 + 0.1528125e-1_f64 * t39626 - t39629 / 4.0_f64 + t39632 / 48.0_f64 - 0.85748036236139473944e-3_f64 * t30866 + 0.22675591804667994222e-1_f64 * t30868 - 0.22675591804667994222e-1_f64 * t30872 + 0.16006300097412701803e-1_f64 * t30874 - 0.80031500487063509016e-2_f64 * t30878 + 0.45017719023973223821e-2_f64 * t30880 + t30883 + 0.21437009059034868486e-3_f64 * t39640 + 0.21437009059034868486e-3_f64 * t39643 + 0.14291339372689912324e-3_f64 * t39647 - 0.85748036236139473945e-2_f64 * t39649;
    t39651
}
