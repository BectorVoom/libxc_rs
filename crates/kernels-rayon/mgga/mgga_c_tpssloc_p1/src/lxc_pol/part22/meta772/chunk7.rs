//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2639/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2639(t11881: f64, t11883: f64, t11904: f64, t1215: f64, t1235: f64, t1244: f64, t1246: f64, t15245: f64, t1755: f64, t18940: f64, t19146: f64, t19201: f64, t22243: f64, t22348: f64, t22365: f64, t22389: f64, t23508: f64, t3610: f64, t3612: f64, t44785: f64, t475: f64, t4964: f64, t5068: f64, t5073: f64, t5076: f64, t52435: f64, t6263: f64, t6265: f64, t73663: f64) -> f64 {
    let t73844 = -t1215 * t22348 * t23508 * t44785 * t475 + t1235 * t1244 * t1246 * t22243 + 6.0_f64 * t1755 * t18940 * t3610 * t3612 + 6.0_f64 * t11881 * t11883 * t73663 + 6.0_f64 * t22389 * t3610 * t5068 + 6.0_f64 * t11904 * t22365 - 3.0_f64 * t15245 * t19146 + 3.0_f64 * t19201 * t5073 + 3.0_f64 * t19201 * t5076 + 3.0_f64 * t4964 * t6265 - 3.0_f64 * t52435 * t6263;
    t73844
}
