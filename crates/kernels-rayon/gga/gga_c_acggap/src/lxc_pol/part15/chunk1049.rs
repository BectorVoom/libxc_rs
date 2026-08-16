//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1049/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1049(t34570: f64, t34592: f64, t34609: f64, t34618: f64, t34620: f64, t34626: f64, t34632: f64, t34659: f64, t34702: f64, t34704: f64, t34710: f64, t34712: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37158 = 0.12862205435420921092e-1_f64 * t34570;
    let t37167 = 11.0_f64 / 96.0_f64 * t34592;
    let t37175 = 11.0_f64 / 96.0_f64 * t34609;
    let t37179 = 0.2264262644851498949e-1_f64 * t34618;
    let t37180 = 0.37737710747524982482e-2_f64 * t34620;
    let t37182 = 0.18868855373762491241e-2_f64 * t34626;
    let t37184 = 0.37737710747524982482e-1_f64 * t34632;
    let t37197 = 7.0_f64 / 36.0_f64 * t34659;
    let t37213 = 0.25724410870841842184e-1_f64 * t34702;
    let t37214 = 0.1543464652250510531e-1_f64 * t34704;
    let t37216 = 0.25724410870841842184e-2_f64 * t34710;
    let t37217 = 0.25724410870841842184e-2_f64 * t34712;
    (t37158, t37167, t37175, t37179, t37180, t37182, t37184, t37197, t37213, t37214, t37216, t37217)
}
