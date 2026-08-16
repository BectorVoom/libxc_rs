//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 826/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk826(t26611: f64, t26678: f64, t858: f64, t25036: f64, t25042: f64, t25047: f64, t25056: f64, t25061: f64, t2597: f64, t26582: f64, t26591: f64, t2713: f64, t4147: f64, t4268: f64, t4273: f64, t7087: f64, t7092: f64, t7107: f64, t7830: f64, t855: f64) -> (f64, f64, f64) {
    let t26679 = t26611 + t26678;
    let t26680 = t858 * t26679;
    let t26684 = -0.82246703342411321825e-2_f64 * t25036 + 2.0_f64 * t855 * t26582 + 2.0_f64 * t2713 * t7830 + 2.0_f64 * t4268 * t7092 + 0.9869604401089358619e-1_f64 * t25042 + 0.3289868133696452873e-1_f64 * t25047 - t26591 + 2.0_f64 * t2597 * t7830 + 0.3289868133696452873e-1_f64 * t25056 + 2.0_f64 * t7087 * t4273 - t855 * t26680 - t4147 * t7107 + 0.16449340668482264365e-1_f64 * t25061;
    (t26679, t26680, t26684)
}
