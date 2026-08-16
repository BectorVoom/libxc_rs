//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3030/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3030(t12153: f64, t4746: f64, t16237: f64, t359: f64, t1024: f64, t1082: f64, t12119: f64, t12143: f64, t12146: f64, t12154: f64, t15670: f64, t15837: f64, t16390: f64, t16406: f64, t16499: f64, t16544: f64, t3204: f64, t3288: f64, t3291: f64, t342: f64, t380: f64, t42261: f64, t43357: f64, t4964: f64, t54955: f64, t55377: f64, t999: f64) -> f64 {
    let t55646 = t4746 * t12153;
    let t55649 = t359 * t16237;
    let t55676 = -0.39512695097613069591e1_f64 * t55646 * t3288 - 0.19756347548806534796e1_f64 * t1024 * t55649 * t999 + 0.65854491829355115987e0_f64 * t342 * t380 * t55377 - 0.11853808529283920877e2_f64 * t42261 * t16499 + 0.13170898365871023197e1_f64 * t3204 * t1082 * t54955 - 0.19756347548806534796e1_f64 * t12146 * t16406 - 0.19756347548806534796e1_f64 * t12154 * t16406 - 0.19756347548806534796e1_f64 * t16544 * t12143 - 0.19756347548806534796e1_f64 * t43357 * t4964 - 0.39512695097613069591e1_f64 * t12154 * t16390 + 0.39512695097613069591e1_f64 * t15670 * t12119 + 0.39512695097613069591e1_f64 * t3204 * t3291 * t15837;
    t55676
}
