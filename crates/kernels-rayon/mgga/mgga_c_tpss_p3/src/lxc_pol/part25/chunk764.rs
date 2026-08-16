//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 764/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk764(t1081: f64, t5177: f64, t3001: f64, t5161: f64, t1054: f64, t1073: f64, t1531: f64, t1543: f64, t2930: f64, t2955: f64, t2974: f64, t2999: f64, t4120: f64, t4158: f64, t421: f64, t5078: f64, t5080: f64, t5084: f64, t5116: f64, t5119: f64, t5124: f64, t5130: f64, t5146: f64, t5149: f64, t5157: f64, t5162: f64) -> (f64, f64, f64) {
    let t5178 = t5177 * t1081;
    let t5181 = t5161 * t3001;
    let t5184 = -0.310907e-1_f64 * t5124 * t421 + 2.0_f64 * t4120 * t1531 - 2.0_f64 * t2930 * t5130 + 1.0_f64 * t1054 * t5146 + 0.32163958997385070134e2_f64 * t2955 * t5149 + t5078 - t5080 + t5084 - t5116 - t5119 - 0.19751673498613801407e-1_f64 * t5157 + 0.11696447245269292414e1_f64 * t4158 * t1543 - 0.11696447245269292414e1_f64 * t2974 * t5162 + 0.5848223622634646207e0_f64 * t1073 * t5178 + 0.17315859105681463759e2_f64 * t2999 * t5181;
    (t5178, t5181, t5184)
}
