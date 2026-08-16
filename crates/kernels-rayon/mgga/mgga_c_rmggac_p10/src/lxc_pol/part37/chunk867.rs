//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 867/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk867(t1525: f64, t1971: f64, t515: f64, t664: f64, t7230: f64, t14124: f64, t14125: f64, t1452: f64, t236: f64, t14131: f64, t8496: f64, t15367: f64, t68524: f64) -> (f64, f64, f64, f64) {
    let t75550 = 0.1064114997332445985e-4_f64 * t7230 * t1971 * t515 * t664 * t1525;
    let t75553 = t14124 * t14125 * t236 * t1452;
    let t75556 = t14131 * t14125 * t8496;
    let t75558 = t68524 * t15367;
    (t75550, t75553, t75556, t75558)
}
