//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2733/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2733(t1250: f64, t5245: f64, t1794: f64, t372: f64, t5277: f64, t17395: f64, t17400: f64, t20809: f64, t12772: f64, t21172: f64, t5331: f64, t3655: f64, t6598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71055 = t1250 * t5245;
    let t71061 = t372 * t5277 * t1794;
    let t71081 = t17400 * t17395;
    let t71112 = t372 * t20809;
    let t71117 = t5331 * t12772 * t21172;
    let t71187 = t6598 * t3655;
    (t71055, t71061, t71081, t71112, t71117, t71187)
}
