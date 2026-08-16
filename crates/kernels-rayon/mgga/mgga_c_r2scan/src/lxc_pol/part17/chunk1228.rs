//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1228/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1228(t37891: f64, t37903: f64, t39816: f64, t41555: f64, t41573: f64, t41574: f64, t41575: f64, t43248: f64, t43251: f64, t43256: f64, t43259: f64, t43262: f64) -> f64 {
    let t44308 = 0.2600466522016280569e0_f64 * t43248 + 0.10401866088065122276e1_f64 * t43251 + t41555 - 0.85366933852867742946e0_f64 * t37891 - 0.31147743054556651237e-1_f64 * t37903 + 0.23804984598836975487e0_f64 * t39816 - 0.5200933044032561138e0_f64 * t43256 - 0.34672886960217074252e0_f64 * t43259 - 0.52009330440325611378e0_f64 * t43262 - t41573 - t41574 - t41575;
    t44308
}
