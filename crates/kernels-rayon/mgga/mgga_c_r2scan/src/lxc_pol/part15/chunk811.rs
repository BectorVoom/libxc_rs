//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 811/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk811(t2747: f64, t468: f64, t1411: f64, t963: f64, t1385: f64, t5034: f64, t4873: f64, t5032: f64, t5039: f64, t7095: f64, t7097: f64, t7108: f64, t7110: f64, t7112: f64, t7126: f64, t7128: f64, t7149: f64, t7150: f64) -> (f64, f64, f64, f64, f64) {
    let t7155 = t2747 * t468;
    let t7156 = 0.11696447245269292414e1_f64 * t7155;
    let t7157 = t963 * t1411;
    let t7158 = 0.5848223622634646207e0_f64 * t7157;
    let t7159 = t963 * t1385;
    let t7160 = 0.17315859105681463759e2_f64 * t7159;
    let t7161 = 0.23392894490538584828e1_f64 * t5034;
    let t7162 = t7095 - t7097 - t7108 + t7110 + t7112 + t7126 + t7128 - t7149 - t7150 + t4873 - t7156 - t7158 - t7160 + t5032 + t7161 + t5039;
    (t7156, t7158, t7160, t7161, t7162)
}
