//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 952/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk952(t344: f64, t381: f64, t225: f64, t1054: f64, t883: f64, t6733: f64, t6686: f64, t6712: f64, t1922: f64, t2966: f64, t1920: f64, t1049: f64, t6703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23328 = t344 * t381;
    let t23329 = t23328 * t225;
    let t23330 = t1054 * t883;
    let t23336 = t6733 * t381;
    let t23346 = t6712 * t6686;
    let t23357 = t2966 * t1922;
    let t23359 = 0.18277045187202515961e-2_f64 * t1920 * t23357;
    let t23365 = t6703 * t1049;
    (t23329, t23330, t23336, t23346, t23359, t23365)
}
