//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 816/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk816<F: Float>(t44: F, t565: F, t7244: F, t6212: F, t938: F, t6211: F, t6475: F, t910: F, t6480: F, t1213: F, t1216: F, t1219: F, t2509: F, t2512: F, t2706: F, t40: F, t6980: F, t903: F, t99: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t7250 = t565 * t7244;
    let t7257 = t6212 * t938;
    let t7258 = t6211 * t7257;
    let t7259 = t6475 * t7258;
    let t7261 = t6212 * t910;
    let t7262 = t6211 * t7261;
    let t7263 = t6480 * t7262;
    let t7276 = piecewise3::<f64>(t45, F::new(0.0), -F::new(10.0) / F::new(27.0) * t2509 * t1213 + F::new(40.0) / F::new(9.0) * t2512 * t6980 + F::new(10.0) / F::new(9.0) * t903 * t1219 + F::new(10.0) / F::new(3.0) * t99 * t1216 - F::new(10.0) * t2706 * t40);
    (t7250, t7257, t7258, t7259, t7261, t7262, t7263, t7276)
}
