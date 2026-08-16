//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2741/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2741(t17290: f64, t5362: f64, t17435: f64, t5327: f64, t3655: f64, t6595: f64, t1256: f64, t21313: f64, t21316: f64, t1261: f64, t20272: f64, t247: f64, t3634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71740 = t17290 * t5362;
    let t71742 = t5327 * t17435;
    let t71744 = t6595 * t3655;
    let t71749 = t21313 * t1256;
    let t71751 = t21316 * t1256;
    let t71827 = t1261 * t247 * t3634 * t20272;
    (t71740, t71742, t71744, t71749, t71751, t71827)
}
