//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 980/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk980(t16521: f64, t8326: f64, t12524: f64, t33193: f64, t4072: f64, t576: f64, t1395: f64, t1458: f64, t7039: f64, t2035: f64, t191: f64, t192: f64, t27215: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120809 = 0.135e2_f64 * t16521 * t8326;
    let t120818 = 27.0_f64 * t12524 * t33193;
    let t120833 = t576 * t4072;
    let t120849 = t1395 * t1458;
    let t121004 = t7039 * t1458;
    let t121007 = t2035 * t4072;
    let t121210 = t27215 * t191 * t192;
    (t120809, t120818, t120833, t120849, t121004, t121007, t121210)
}
