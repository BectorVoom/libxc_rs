//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2807/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2807(t4321: f64, t6049: f64, t689: f64, t4481: f64, t63084: f64, t18323: f64, t23383: f64, t2770: f64, t40970: f64, t40978: f64, t50161: f64, t50214: f64, t50219: f64, t50221: f64, t50223: f64, t50240: f64, t61385: f64, t61397: f64, t61400: f64, t61403: f64, t61407: f64, t865: f64, t886: f64) -> f64 {
    let t75998 = t689 * t4321 * t6049;
    let t76010 = t63084 * t4481;
    let t76012 = 0.16463622957338778996e-1_f64 * t61385 - 0.11853808529283920877e2_f64 * t50240 * t50161 * t18323 - 0.26019841438354088051e-2_f64 * t40970 - 0.32927245914677557992e-1_f64 * t75998 - 0.19637199382202157274e-3_f64 * t40978 - 0.13878983423218070567e-1_f64 * t50214 - t50219 - t50221 - t50223 - 0.39029762157531132074e-2_f64 * t61397 + 0.39029762157531132074e-2_f64 * t61400 - 0.32927245914677557992e-1_f64 * t61403 + 0.13170898365871023197e1_f64 * t865 * t2770 * t23383 * t886 + 0.69394917116090352834e-2_f64 * t61407 - 0.29272321618148349057e-1_f64 * t76010;
    t76012
}
