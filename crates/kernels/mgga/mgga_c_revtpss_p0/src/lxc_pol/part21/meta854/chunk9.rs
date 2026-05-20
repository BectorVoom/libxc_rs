//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3232/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3232<F: Float>(t1234: F, t1269: F, t12699: F, t12709: F, t12723: F, t1280: F, t1281: F, t1285: F, t1287: F, t16756: F, t16763: F, t16768: F, t17170: F, t17178: F, t17188: F, t17289: F, t17829: F, t17875: F, t17880: F, t17949: F, t17951: F, t3666: F, t3746: F, t3763: F, t45852: F, t5478: F, t5491: F, t56376: F, t57536: F, t59032: F) -> F {
    let t59983 = F::cast_from(0.19756347548806534796e1_f64) * t3746 * t16763 - F::cast_from(0.39512695097613069591e1_f64) * t12709 * t17829 + F::cast_from(0.79025390195226139182e1_f64) * t45852 * t17188 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t1269 * t17170 * t1287 - F::cast_from(0.39512695097613069591e1_f64) * t17880 * t17178 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t16756 * t17875 - F::cast_from(0.19756347548806534796e1_f64) * t17289 * t3763 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t16768 - F::cast_from(0.19756347548806534796e1_f64) * t59032 * t1281 - F::cast_from(0.39512695097613069591e1_f64) * t12723 * t17829 + F::cast_from(0.19756347548806534796e1_f64) * t17949 * t57536 * t17951 + F::cast_from(0.19756347548806534796e1_f64) * t12699 * t5491 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1280 * t56376;
    t59983
}
