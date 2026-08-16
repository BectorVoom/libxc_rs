//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 816/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk816(t44: f64, t565: f64, t7244: f64, t6212: f64, t938: f64, t6211: f64, t6475: f64, t910: f64, t6480: f64, t1213: f64, t1216: f64, t1219: f64, t2509: f64, t2512: f64, t2706: f64, t40: f64, t6980: f64, t903: f64, t99: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t7250 = t565 * t7244;
    let t7257 = t6212 * t938;
    let t7258 = t6211 * t7257;
    let t7259 = t6475 * t7258;
    let t7261 = t6212 * t910;
    let t7262 = t6211 * t7261;
    let t7263 = t6480 * t7262;
    let t7276 = piecewise3(t45, 0.0_f64, -10.0_f64 / 27.0_f64 * t2509 * t1213 + 40.0_f64 / 9.0_f64 * t2512 * t6980 + 10.0_f64 / 9.0_f64 * t903 * t1219 + 10.0_f64 / 3.0_f64 * t99 * t1216 - 10.0_f64 * t2706 * t40);
    (t7250, t7257, t7258, t7259, t7261, t7262, t7263, t7276)
}
