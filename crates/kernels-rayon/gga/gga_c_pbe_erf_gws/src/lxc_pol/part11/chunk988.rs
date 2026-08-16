//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 988/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk988(t11159: f64, t713: f64, t1923: f64, t256: f64, t3583: f64, t10606: f64, t723: f64, t1903: f64, t3584: f64, t10610: f64, t1918: f64, t1617: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34395 = t11159 * t713;
    let t34418 = t3583 * t1923 * t256;
    let t34500 = t10606 * t723;
    let t34538 = t3584 * t1903;
    let t34544 = t10610 * t1918;
    let t34565 = t3603 * t1617;
    (t34395, t34418, t34500, t34538, t34544, t34565)
}
