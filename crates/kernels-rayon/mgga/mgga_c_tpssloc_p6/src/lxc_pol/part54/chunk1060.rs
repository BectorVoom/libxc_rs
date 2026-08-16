//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1060/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1060(t26728: f64, t4272: f64, t2047: f64, t4142: f64, t1492: f64, t7084: f64, t13042: f64, t13053: f64, t13065: f64, t2054: f64, t23250: f64, t23254: f64, t24318: f64, t24321: f64, t25168: f64, t25339: f64, t25343: f64, t259: f64, t26722: f64, t26726: f64) -> (f64, f64, f64, f64) {
    let t26729 = t26728 * t4272;
    let t26732 = t4142 * t2047;
    let t26734 = t1492 * t7084;
    let t26737 = -t23250 + t24318 - 0.82246703342411321825e-2_f64 * t23254 + t24321 - t13065 * t2054 + t26722 * t259 - 0.3289868133696452873e-1_f64 * t25339 - 0.3289868133696452873e-1_f64 * t25343 + t26726 - t13042 * t2054 - 6.0_f64 * t25168 * t26729 + t26732 * t259 + t26734 * t259 - t13053 * t2054;
    (t26729, t26732, t26734, t26737)
}
