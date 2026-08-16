//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1140/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1140(t1062: f64, t15692: f64, t1072: f64, t5156: f64, t1081: f64, t15351: f64, t1053: f64, t5124: f64, t1054: f64, t1063: f64, t1073: f64, t1082: f64, t12070: f64, t1543: f64, t15607: f64, t15609: f64, t2925: f64, t2969: f64, t4158: f64, t4181: f64, t5146: f64, t5149: f64, t5162: f64, t5178: f64, t5181: f64, t9359: f64, t9370: f64, t9419: f64) -> f64 {
    let t15693 = t15692 * t1062;
    let t15698 = t5156 * t1072;
    let t15709 = t15351 * t1081;
    let t15714 = t5124 * t1053;
    let t15717 = 1.0_f64 * t2925 * t5146 + 1.0_f64 * t1054 * t15693 + 0.32163958997385070134e2_f64 * t9419 * t5149 + 0.5848223622634646207e0_f64 * t15698 * t1082 + 0.11696447245269292414e1_f64 * t12070 * t1543 + 0.11696447245269292414e1_f64 * t4158 * t4181 - 0.11696447245269292414e1_f64 * t9359 * t5162 + 0.5848223622634646207e0_f64 * t2969 * t5178 + 0.5848223622634646207e0_f64 * t1073 * t15709 + 0.17315859105681463759e2_f64 * t9370 * t5181 + 1.0_f64 * t15714 * t1063 + t15607 - t15609;
    t15717
}
