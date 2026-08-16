//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1273/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1273(t37393: f64, t37401: f64, t39059: f64, t39061: f64, t39062: f64, t39064: f64, t43838: f64, t43842: f64, t44120: f64, t44122: f64, t44125: f64, t44127: f64, t44129: f64, t44132: f64, t44135: f64) -> f64 {
    let t44971 = -t44120 + t44122 + 0.72042316457491791901e-3_f64 * t43838 - 0.10248087766267884741e-3_f64 * t43842 + t44125 - t44127 - t44129 - t44132 - 0.86737941314158990616e-4_f64 * t37393 - t39059 + 0.92232789896410962673e-3_f64 * t37401 + t44135 + t39061 + t39062 - t39064;
    t44971
}
