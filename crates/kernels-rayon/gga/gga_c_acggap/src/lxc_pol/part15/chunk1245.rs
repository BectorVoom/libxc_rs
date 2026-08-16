//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1245/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1245(t35486: f64, t35499: f64, t37566: f64, t37567: f64, t37569: f64, t37573: f64, t37576: f64, t39985: f64, t39987: f64, t39990: f64, t39995: f64, t39999: f64, t40003: f64, t40005: f64, t40009: f64, t40011: f64, t40015: f64, t40019: f64) -> f64 {
    let t41895 = t37566 + t37567 - t37569 + 0.85748036236139473944e-3_f64 * t39985 + 0.12579236915841660828e-2_f64 * t39987 + 0.12579236915841660828e-2_f64 * t39990 - 0.51448821741683684367e-2_f64 * t35486 + 0.62896184579208304138e-3_f64 * t39995 - 0.18868855373762491241e-1_f64 * t39999 - 0.15095084299009992993e-1_f64 * t40003 + 0.11321313224257494745e-1_f64 * t40005 - 0.62896184579208304138e-3_f64 * t40009 + t37573 - 0.75475421495049964968e-2_f64 * t40011 + 0.62896184579208304138e-3_f64 * t40015 - 0.12862205435420921092e-1_f64 * t40019 - t35499 - t37576;
    t41895
}
