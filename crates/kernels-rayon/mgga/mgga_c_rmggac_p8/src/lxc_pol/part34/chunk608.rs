//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 608/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk608(t15397: f64, t2067: f64, t3369: f64, t14230: f64, t209: f64, t605: f64, t664: f64, t515: f64, t1971: f64, t1970: f64, t26: f64, t14163: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15398 = t2067 * t15397;
    let t15399 = t3369 * t15398;
    let t15400 = t14230 * t15399;
    let t15403 = t664 * t605 * t209;
    let t15404 = t515 * t15403;
    let t15405 = t1971 * t15404;
    let t15406 = t1970 * t15405;
    let t15409 = t26 * t605 * t209;
    let t15410 = t2067 * t15409;
    let t15411 = t3369 * t15410;
    let t15412 = t14163 * t15411;
    (t15399, t15400, t15405, t15406, t15411, t15412)
}
