//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1238/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1238(t172: f64, t6320: f64, t763: f64, t15972: f64, t12097: f64, t12106: f64, t12111: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t12116: f64, t12118: f64, t15976: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19681 = t6320 * t172;
    let t19682 = t19681 * t763;
    let t19683 = 0.5848223622634646207e0_f64 * t19682;
    let t19684 = 2.0_f64 * t15972;
    let t19685 = 0.24415263074675393405e-3_f64 * t12097;
    let t19686 = 8.0_f64 * t12106;
    let t19687 = 0.10843581300301739842e-1_f64 * t12111;
    let t19688 = t9793 + t9797 - t9820 - t9824 - t19683 + t19684 + t19685 + t12103 - t12105 - t19686 - t12109 + t19687 - t12114 + t12116 + t12118 - t15976;
    (t19683, t19684, t19685, t19686, t19687, t19688)
}
