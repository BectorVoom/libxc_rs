//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1263/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1263(t11063: f64, t11064: f64, t11067: f64, t11100: f64, t11101: f64, t1134: f64, t18258: f64, t2118: f64, t21346: f64, t2964: f64, t2989: f64, t307: f64, t30843: f64, t30925: f64, t30977: f64, t3694: f64, t7824: f64, t786: f64, t790: f64, t799: f64, t9647: f64, t9712: f64, t9713: f64) -> f64 {
    let t30982 = -0.19756347548806534796e1_f64 * t1134 * t9713 - 0.39512695097613069591e1_f64 * t786 * t11064 + 0.15805078039045227836e2_f64 * t307 * t18258 * t11063 * t799 - 0.11853808529283920877e2_f64 * t307 * t9647 * t2989 + 0.39512695097613069591e1_f64 * t786 * t11067 - 0.11853808529283920877e2_f64 * t21346 * t30843 * t799 + 0.39512695097613069591e1_f64 * t307 * t7824 * t3694 + 0.39512695097613069591e1_f64 * t307 * t2964 * t9712 - 0.65854491829355115987e0_f64 * t786 * t11101 + 0.13170898365871023197e1_f64 * t307 * t2118 * t11100 * t799 - 0.65854491829355115987e0_f64 * t307 * t790 * (t30925 + t30977);
    t30982
}
