//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3030/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3030<F: Float>(t12153: F, t4746: F, t16237: F, t359: F, t1024: F, t1082: F, t12119: F, t12143: F, t12146: F, t12154: F, t15670: F, t15837: F, t16390: F, t16406: F, t16499: F, t16544: F, t3204: F, t3288: F, t3291: F, t342: F, t380: F, t42261: F, t43357: F, t4964: F, t54955: F, t55377: F, t999: F) -> F {
    let t55646 = t4746 * t12153;
    let t55649 = t359 * t16237;
    let t55676 = -F::cast_from(0.39512695097613069591e1_f64) * t55646 * t3288 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t55649 * t999 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t380 * t55377 - F::cast_from(0.11853808529283920877e2_f64) * t42261 * t16499 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t1082 * t54955 - F::cast_from(0.19756347548806534796e1_f64) * t12146 * t16406 - F::cast_from(0.19756347548806534796e1_f64) * t12154 * t16406 - F::cast_from(0.19756347548806534796e1_f64) * t16544 * t12143 - F::cast_from(0.19756347548806534796e1_f64) * t43357 * t4964 - F::cast_from(0.39512695097613069591e1_f64) * t12154 * t16390 + F::cast_from(0.39512695097613069591e1_f64) * t15670 * t12119 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t3291 * t15837;
    t55676
}
