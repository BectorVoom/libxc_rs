//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 726/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk726(t225: f64, t4149: f64, t3701: f64, t5356: f64, t12461: f64, t1845: f64, t5213: f64, t5211: f64, t1372: f64, t1824: f64, t5286: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13463 = t4149 * t225;
    let t15868 = t5356 * t3701;
    let t15899 = t1845 * t12461;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16036 = t1372 * t1824;
    let t16040 = t562 * t5286;
    (t13463, t15868, t15899, t16022, t16030, t16036, t16040)
}
