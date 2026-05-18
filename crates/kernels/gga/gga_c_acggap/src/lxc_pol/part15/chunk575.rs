//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 575/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk575<F: Float>(t1576: F, t997: F, t4210: F, t535: F, t1181: F, t1163: F, t1165: F, t530: F, t3194: F, t540: F, t1005: F, t1423: F) -> (F, F, F, F, F, F, F, F) {
    let t4957 = F::new(0.40015750243531754508e-2) * t997 * t1576;
    let t4958 = t535 * t4210;
    let t4959 = t1181 * t4958;
    let t4961 = F::new(0.85748036236139473944e-3) * t1163 * t4959;
    let t4967 = t1165 * t530 * t4210;
    let t4969 = F::new(0.17149607247227894789e-2) * t3194 * t4967;
    let t4987 = t1165 * t540 * t4210;
    let t4989 = F::new(0.85748036236139473944e-3) * t1163 * t4987;
    let t4994 = t1005 * t1423;
    (t4957, t4959, t4961, t4967, t4969, t4987, t4989, t4994)
}
