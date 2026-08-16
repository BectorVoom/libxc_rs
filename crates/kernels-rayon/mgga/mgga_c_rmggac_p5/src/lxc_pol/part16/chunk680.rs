//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 680/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk680(t515: f64, t9843: f64, t1971: f64, t7230: f64, t2310: f64, t8571: f64, t2320: f64, t9222: f64, t2295: f64, t6355: f64, t1704: f64, t27: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9844 = t515 * t9843;
    let t9845 = t1971 * t9844;
    let t9846 = t7230 * t9845;
    let t9848 = t8571 * t2310;
    let t9850 = t9222 * t2320;
    let t9861 = t6355 * t2295;
    let t9864 = t27 * t649 * t1704;
    (t9845, t9846, t9848, t9850, t9861, t9864)
}
