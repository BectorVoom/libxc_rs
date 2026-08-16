//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1098/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1098(t20199: f64, t2590: f64, t2595: f64, t17053: f64, t2602: f64, t2587: f64, t5264: f64, t2655: f64, t1730: f64, t2648: f64, t16324: f64, t177: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20201 = t2590 * t20199 * t2595;
    let t20202 = 0.34013387707001991332e-1_f64 * t20201;
    let t20205 = t17053 * t2602;
    let t20221 = t5264 * t2587;
    let t20222 = 35.0_f64 / 72.0_f64 * t20221;
    let t20242 = t17053 * t2655;
    let t20261 = t1730 * t20199 * t2648;
    let t20262 = 0.17006693853500995666e-1_f64 * t20261;
    let t20267 = t16324 * t177;
    (t20202, t20205, t20222, t20242, t20262, t20267)
}
