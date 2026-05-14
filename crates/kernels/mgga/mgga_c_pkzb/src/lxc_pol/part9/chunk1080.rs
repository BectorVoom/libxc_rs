//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1080/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1080<F: Float>(t1045: F, t1055: F, t158: F, t1784: F, t1791: F, t1792: F, t1812: F, t1813: F, t184: F, t188: F, t20441: F, t20498: F, t20553: F, t2671: F, t2678: F, t2679: F, t2702: F, t2703: F, t5408: F, t5418: F, t5420: F, t5462: F, t626: F, t7116: F) -> (F,) {
    let t20567 = 0.39512695097613069591e1 * t1784 * t2679 + 0.39512695097613069591e1 * t184 * t7116 * t1812 - 0.19756347548806534796e1 * t2671 * t1813 - 0.11853808529283920877e2 * t184 * t5418 * t2702 * t1791 + 0.65854491829355115987e0 * t20441 * t158 * t188 - 0.19756347548806534796e1 * t1784 * t2703 - 0.65854491829355115987e0 * t184 * t626 * (t20498 + t20553) - 0.65854491829355115987e0 * t5408 * t1055 - 0.39512695097613069591e1 * t1045 * t5420 + 0.13170898365871023197e1 * t184 * t2678 * t5462 + 0.39512695097613069591e1 * t2671 * t1792;
    (t20567,)
}
