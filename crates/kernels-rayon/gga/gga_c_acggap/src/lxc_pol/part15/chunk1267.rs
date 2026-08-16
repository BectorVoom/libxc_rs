//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1267/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1267(t36385: f64, t36386: f64, t36390: f64, t36392: f64, t37987: f64, t37988: f64, t37992: f64, t40553: f64, t40557: f64, t40561: f64, t40565: f64, t40567: f64, t40569: f64, t40573: f64, t40576: f64, t40579: f64, t40584: f64, t40587: f64) -> f64 {
    let t42175 = 0.37737710747524982482e-2_f64 * t40553 + 0.37737710747524982482e-2_f64 * t40557 + 0.37737710747524982482e-2_f64 * t40561 + 0.25158473831683321655e-2_f64 * t40565 + 0.37737710747524982483e-2_f64 * t40567 - 0.183375e0_f64 * t40569 + t37987 - t40573 / 192.0_f64 + t37988 + 0.1528125e-1_f64 * t40576 + t40579 / 16.0_f64 - t36385 + 0.11181543925192587402e-1_f64 * t36386 + t37992 + 0.75475421495049964965e-2_f64 * t36390 + 0.68598428988911579156e-2_f64 * t36392 + 0.62896184579208304136e-2_f64 * t40584 + 0.37737710747524982482e-2_f64 * t40587;
    t42175
}
