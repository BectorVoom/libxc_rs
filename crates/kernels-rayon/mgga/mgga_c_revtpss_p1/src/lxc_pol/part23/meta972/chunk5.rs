//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3296/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296(t1385: f64, t22964: f64, t1399: f64, t14230: f64, t14255: f64, t1883: f64, t22009: f64, t49238: f64, t49256: f64, t49274: f64, t5675: f64, t5745: f64, t6844: f64, t74886: f64, t75141: f64, t75145: f64, t75147: f64, t820: f64, t86470: f64) -> f64 {
    let t86552 = t1385 * t22964;
    let t86556 = 0.11708928647259339623e0_f64 * t75141 + 0.19514881078765566038e-2_f64 * t49238 + 0.43902994552903410656e-1_f64 * t75145 - 0.43902994552903410656e-1_f64 * t75147 + 0.11853808529283920877e2_f64 * t5745 * t22009 * t14230 - 0.19756347548806534796e1_f64 * t820 * t14255 * t6844 - 0.78059524315062264152e-1_f64 * t49256 + 0.11853808529283920877e2_f64 * t5745 * t86470 * t5675 - 0.19756347548806534796e1_f64 * t820 * t74886 * t1883 - 0.65854491829355115987e0_f64 * t820 * t86552 * t1399 - t49274;
    t86556
}
