//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 846/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk846(t225: f64, t4149: f64, t4947: f64, t4943: f64, t4941: f64, t5053: f64, t3701: f64, t5356: f64, t12461: f64, t1845: f64, t5213: f64, t5211: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13463 = t4149 * t225;
    let t14972 = t4947 * t225;
    let t14980 = t4943 * t225;
    let t15797 = t4941 * t225;
    let t15820 = t5053 * t225;
    let t15868 = t5356 * t3701;
    let t15899 = t1845 * t12461;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    (t13463, t14972, t14980, t15797, t15820, t15868, t15899, t16022, t16030)
}
