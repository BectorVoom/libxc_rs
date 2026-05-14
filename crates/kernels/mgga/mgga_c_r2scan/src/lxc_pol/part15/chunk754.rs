//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 754/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk754<F: Float>(t2747: F, t468: F, t1411: F, t963: F, t1385: F, t5034: F, t4873: F, t5032: F, t5039: F, t7095: F, t7097: F, t7108: F, t7110: F, t7112: F, t7126: F, t7128: F, t7149: F, t7150: F) -> (F, F, F, F, F) {
    let t7155 = t2747 * t468;
    let t7156 = 0.11696447245269292414e1 * t7155;
    let t7157 = t963 * t1411;
    let t7158 = 0.5848223622634646207e0 * t7157;
    let t7159 = t963 * t1385;
    let t7160 = 0.17315859105681463759e2 * t7159;
    let t7161 = 0.23392894490538584828e1 * t5034;
    let t7162 = t7095 - t7097 - t7108 + t7110 + t7112 + t7126 + t7128 - t7149 - t7150 + t4873 - t7156 - t7158 - t7160 + t5032 + t7161 + t5039;
    (t7156, t7158, t7160, t7161, t7162)
}
