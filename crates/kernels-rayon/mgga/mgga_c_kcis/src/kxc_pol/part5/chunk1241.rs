//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1241/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1241(t20157: f64, t20160: f64, t20162: f64, t20165: f64, t20167: f64, t20170: f64, t20174: f64, t20176: f64, t20179: f64, t20181: f64, t20183: f64, t20186: f64, t20188: f64, t20192: f64, t20195: f64, t20198: f64, t20201: f64, t20203: f64, t20206: f64) -> f64 {
    let t20809 = -0.9375e-1_f64 * t20157 + 0.375e0_f64 * t20160 + 0.26979166666666666666e-1_f64 * t20162 + 0.25e0_f64 * t20165 + 0.1875e0_f64 * t20167 + 0.89930555555555555553e-2_f64 * t20170 + 0.101171875e-1_f64 * t20174 - 0.13489583333333333333e-1_f64 * t20176 - 0.625e-1_f64 * t20179 + 0.625e-1_f64 * t20181 - 0.1875e0_f64 * t20183 + 0.55555555555555555555e-1_f64 * t20186 - 0.13489583333333333333e-1_f64 * t20188 - 0.13489583333333333333e-1_f64 * t20192 - 0.53958333333333333333e-1_f64 * t20195 + 0.44965277777777777777e-2_f64 * t20198 + 0.13489583333333333333e-1_f64 * t20201 + 0.14388888888888888889e0_f64 * t20203 + 0.13489583333333333333e-1_f64 * t20206;
    t20809
}
