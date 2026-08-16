//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 980/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk980<F: Float>(t16521: F, t8326: F, t12524: F, t33193: F, t4072: F, t576: F, t1395: F, t1458: F, t7039: F, t2035: F, t191: F, t192: F, t27215: F) -> (F, F, F, F, F, F, F) {
    let t120809 = F::cast_from(0.135e2_f64) * t16521 * t8326;
    let t120818 = F::cast_from(27.0_f64) * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t121004 = t7039 * t1458;
    let t121007 = t2035 * t4072;
    let t121210 = t27215 * t191 * t192;
    (t120809, t120818, t120833, t120849, t121004, t121007, t121210)
}
