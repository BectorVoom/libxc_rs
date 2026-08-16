//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2668/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2668(t53777: f64, t53779: f64, t53782: f64, t53787: f64, t19681: f64, t2528: f64, t172: f64, t19572: f64, t763: f64, t2535: f64, t40611: f64, t6324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56092 = 0.43374325201206959368e-1_f64 * t53777;
    let t56093 = 0.65061487801810439052e-1_f64 * t53779;
    let t56094 = 0.43374325201206959368e-1_f64 * t53782;
    let t56098 = 0.97661052298701573622e-3_f64 * t53787;
    let t56099 = t19681 * t2528;
    let t56100 = 0.17315859105681463759e2_f64 * t56099;
    let t56102 = t19572 * t172 * t763;
    let t56103 = 0.11696447245269292414e1_f64 * t56102;
    let t56104 = t19681 * t2535;
    let t56105 = 0.5848223622634646207e0_f64 * t56104;
    let t56106 = t6324 * t40611;
    (t56092, t56093, t56094, t56098, t56100, t56103, t56105, t56106)
}
