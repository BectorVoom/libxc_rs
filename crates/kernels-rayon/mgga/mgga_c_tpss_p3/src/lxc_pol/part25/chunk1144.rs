//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1144/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1144(t12075: f64, t12086: f64, t15737: f64, t15740: f64, t15744: f64, t15751: f64, t15754: f64, t15757: f64, t15760: f64, t15764: f64, t15767: f64, t15771: f64, t15788: f64, t2955: f64, t2974: f64, t2999: f64, t4163: f64, t4185: f64, t421: f64, t9373: f64, t9380: f64, t9465: f64) -> f64 {
    let t15791 = 0.32163958997385070134e2_f64 * t2955 * t15737 + 0.64327917994770140268e2_f64 * t2955 * t15740 + 0.2069040516770936012e4_f64 * t9465 * t15744 - 0.23392894490538584828e1_f64 * t12086 * t4163 + 0.34631718211362927517e2_f64 * t12075 * t4185 + 0.35089341735807877242e1_f64 * t2999 * t15751 - 0.23392894490538584828e1_f64 * t2974 * t15754 - 0.10389515463408878255e3_f64 * t9373 * t15757 - 0.11696447245269292414e1_f64 * t2974 * t15760 + 0.17315859105681463759e2_f64 * t2999 * t15764 + 0.34631718211362927518e2_f64 * t2999 * t15767 + 0.10254018858216406658e4_f64 * t9380 * t15771 - 0.310907e-1_f64 * t15788 * t421;
    t15791
}
