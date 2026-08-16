//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1112/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1112(t29120: f64, t29143: f64, t29170: f64, t29184: f64, t2205: f64, t6860: f64, t29045: f64, t29047: f64, t29049: f64, t29052: f64, t29054: f64, t29057: f64, t29060: f64, t29063: f64, t29065: f64, t29067: f64, t29069: f64, t29071: f64, t29073: f64, t29075: f64, t29077: f64, t29079: f64) -> (f64, f64, f64) {
    let t29186 = t29120 + t29143 + t29170 + t29184;
    let t29188 = t2205 * t6860;
    let t29214 = 0.9375e-1_f64 * t29045 - 0.1875e0_f64 * t29047 + 0.125e0_f64 * t29049 + 0.1875e0_f64 * t29052 - 0.125e0_f64 * t29054 - 0.9375e-1_f64 * t29057 - 0.20833333333333333333e-1_f64 * t29060 + 0.625e-1_f64 * t29063 - 0.20234375e-1_f64 * t29065 + 0.4046875e-1_f64 * t29067 - 0.53958333333333333334e-1_f64 * t29069 - 0.4046875e-1_f64 * t29071 + 0.53958333333333333334e-1_f64 * t29073 + 0.20234375e-1_f64 * t29075 - 0.89930555555555555557e-2_f64 * t29077 - 0.26979166666666666667e-1_f64 * t29079;
    (t29186, t29188, t29214)
}
