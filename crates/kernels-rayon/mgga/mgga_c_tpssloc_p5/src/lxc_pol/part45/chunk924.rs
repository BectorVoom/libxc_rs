//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 924/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk924(t112702: f64, t6662: f64, t857: f64, t22986: f64, t23270: f64, t776: f64, t30667: f64, t6547: f64, t23222: f64, t30663: f64, t6552: f64, t1880: f64, t23196: f64) -> (f64, f64, f64, f64, f64) {
    let t112703 = 0.3289868133696452873e-1_f64 * t112702;
    let t112719 = t857 * t6662;
    let t112723 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t112719 * t776;
    let t112726 = t6547 * t30667;
    let t112727 = 0.76763589786250567036e-1_f64 * t112726;
    let t112730 = 0.3289868133696452873e-1_f64 * t6552 * t30663 * t23222;
    let t112733 = 0.3289868133696452873e-1_f64 * t1880 * t30663 * t23196;
    (t112703, t112723, t112727, t112730, t112733)
}
