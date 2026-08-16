//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1009/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1009(t1550: f64, t699: f64, t8708: f64, t75443: f64, t15450: f64, t7255: f64, t1970: f64, t1971: f64, t209: f64, t2227: f64, t515: f64, t605: f64) -> (f64, f64, f64, f64) {
    let t77604 = t1550 * t699 * t8708;
    let t77605 = 0.79828278012425390427e-1_f64 * t77604;
    let t77606 = 0.54549323308490683456e-1_f64 * t75443;
    let t77607 = t7255 * t15450;
    let t77608 = 0.42564599893297839398e-5_f64 * t77607;
    let t77613 = t1970 * t1971 * t515 * t2227 * t605 * t209;
    (t77605, t77606, t77608, t77613)
}
