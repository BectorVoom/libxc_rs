//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 876/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk876(t6534: f64, t7230: f64, t12524: f64, t8657: f64, t20173: f64, t1873: f64, t7056: f64, t3941: f64, t2039: f64, t191: f64, t192: f64, t7412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31803 = 0.135e2_f64 * t7230 * t6534;
    let t31811 = 27.0_f64 * t12524 * t8657;
    let t31813 = 27.0_f64 * t20173 * t8657;
    let t31814 = t7056 * t1873;
    let t31816 = 27.0_f64 * t3941 * t31814;
    let t31817 = t2039 * t6534;
    let t31819 = 27.0_f64 * t3941 * t31817;
    let t31832 = t7412 * t191 * t192;
    (t31803, t31811, t31813, t31814, t31816, t31817, t31819, t31832)
}
