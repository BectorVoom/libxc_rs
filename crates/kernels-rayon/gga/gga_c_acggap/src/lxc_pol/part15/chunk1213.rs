//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1213/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1213(t34077: f64, t34127: f64, t34132: f64, t34156: f64, t36934: f64, t36937: f64, t36938: f64, t36939: f64, t36940: f64, t36942: f64, t36951: f64, t39026: f64, t39029: f64, t39031: f64, t39035: f64, t39039: f64, t39041: f64, t39043: f64) -> f64 {
    let t41452 = t34077 - t36934 - 0.21437009059034868486e-3_f64 * t39026 - 0.21437009059034868486e-3_f64 * t39029 + 0.18868855373762491241e-1_f64 * t39031 + 0.85748036236139473944e-3_f64 * t39035 - 0.31448092289604152068e-2_f64 * t39039 + 11.0_f64 / 192.0_f64 * t39041 + 11.0_f64 / 576.0_f64 * t39043 + t36937 - t36938 - t36939 + t36940 + t36942 + 0.57165357490759649296e-3_f64 * t34127 + t36951 - 0.75475421495049964964e-2_f64 * t34132 - 0.37737710747524982482e-2_f64 * t34156;
    t41452
}
