//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 612/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk612(t201: f64, t8607: f64, t1979: f64, t1982: f64, t2320: f64, t7691: f64, t128: f64, t1525: f64, t118: f64, t1986: f64, t1994: f64, t22: f64, t7262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8608 = t8607 * t201;
    let t8610 = t8608 * t1979 * t1982;
    let t8612 = t7691 * t2320;
    let t8614 = t128 * t1525;
    let t8615 = t118 * t8614;
    let t8616 = t1986 * t8615;
    let t8617 = t1994 * t8616;
    let t8619 = t7262 * t22;
    (t8608, t8610, t8612, t8616, t8617, t8619)
}
