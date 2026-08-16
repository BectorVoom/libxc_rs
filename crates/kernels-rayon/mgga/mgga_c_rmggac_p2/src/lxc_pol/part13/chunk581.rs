//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 581/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk581(t1268: f64, t1986: f64, t675: f64, t1990: f64, t2191: f64, t1274: f64, t1173: f64, t2189: f64, t674: f64, t1997: f64, t1240: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7682 = t1986 * t1268;
    let t7683 = t675 * t7682;
    let t7685 = t2191 * t1990;
    let t7687 = t1986 * t1274;
    let t7688 = t675 * t7687;
    let t7690 = t2189 * t1173;
    let t7691 = t7690 * t674;
    let t7692 = t7691 * t1997;
    let t7694 = t128 * t1240;
    (t7682, t7683, t7685, t7687, t7688, t7690, t7691, t7692, t7694)
}
