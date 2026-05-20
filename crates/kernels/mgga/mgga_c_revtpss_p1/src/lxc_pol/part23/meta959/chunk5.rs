//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3226/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3226<F: Float>(t1284: F, t24698: F, t12751: F, t1285: F, t1287: F, t1288: F, t17192: F, t17958: F, t20703: F, t20850: F, t21448: F, t21480: F, t21483: F, t21484: F, t21512: F, t21595: F, t24931: F, t25009: F, t3670: F, t3746: F, t3782: F, t3783: F, t45769: F, t45859: F, t460: F, t487: F, t489: F, t5486: F, t5487: F, t59749: F, t82422: F, t84203: F, t84415: F, t84457: F) -> F {
    let t84859 = t24698 * t1284;
    let t84887 = F::cast_from(0.79025390195226139182e1_f64) * t45859 * t84457 * t21483 - F::cast_from(0.79025390195226139182e1_f64) * t12751 * t21512 * t21595 + F::cast_from(0.65854491829355115987e0_f64) * t84859 * t1288 + F::cast_from(0.65854491829355115987e0_f64) * t3746 * t25009 + F::cast_from(0.39512695097613069591e1_f64) * t45769 * t24931 + F::cast_from(0.39512695097613069592e1_f64) * t3670 * t5486 * t20703 - F::cast_from(0.19756347548806534796e1_f64) * t17192 * t21480 - F::cast_from(0.39512695097613069591e1_f64) * t59749 * t21484 - F::cast_from(0.19756347548806534796e1_f64) * t20850 * t5487 - F::cast_from(0.39512695097613069591e1_f64) * t17958 * t21448 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t487 * t82422 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t3782 * t84415 * t3783 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t489 * t84203;
    t84887
}
