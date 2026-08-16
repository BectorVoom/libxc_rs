//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 890/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk890(t574: f64, t638: f64, t639: f64, t8849: f64, t1656: f64, t2338: f64, t1550: f64, t2060: f64, t30400: f64, t194: f64, t1979: f64, t1982: f64, t201: f64, t6070: f64) -> (f64, f64, f64, f64) {
    let t44925 = t638 * t639 * t8849 * t574;
    let t44929 = t638 * t639 * t2338 * t1656;
    let t44941 = t1550 * t2060 * t30400;
    let t44949 = t194 * t6070 * t201 * t1979 * t1982;
    (t44925, t44929, t44941, t44949)
}
