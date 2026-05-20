//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3224/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3224<F: Float>(t1214: F, t1248: F, t12717: F, t12723: F, t1285: F, t1287: F, t13127: F, t13129: F, t17854: F, t1811: F, t20850: F, t20900: F, t21439: F, t21607: F, t24989: F, t3755: F, t45659: F, t5284: F, t5436: F, t5449: F, t5474: F, t5491: F, t59871: F, t59872: F, t6695: F, t82293: F, t82886: F, t82899: F, t83662: F, t84462: F) -> F {
    let t84816 = -F::cast_from(0.23707617058567841754e2_f64) * t59871 * t82886 * t59872 * t1248 + F::cast_from(0.39512695097613069591e1_f64) * t45659 * t82293 * t17854 * t1214 + F::cast_from(0.19756347548806534796e1_f64) * t21439 * t5474 - F::cast_from(0.19756347548806534796e1_f64) * t20850 * t5449 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t1811 * t20900 * t1287 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t6695 * t5284 * t1287 - F::cast_from(0.19756347548806534796e1_f64) * t12723 * t24989 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t82899 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t13127 * t84462 * t13129 + F::cast_from(0.39512695097613069591e1_f64) * t5436 * t21607 + F::cast_from(0.19756347548806534796e1_f64) * t21439 * t5491 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t83662 * t1287;
    t84816
}
