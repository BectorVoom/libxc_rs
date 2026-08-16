//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1036/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1036(t1105: f64, t1109: f64, t3886: f64, t8589: f64, t829: f64, t830: f64, t376: f64, t3772: f64, t13173: f64, t2366: f64, t833: f64, t13207: f64, t4414: f64) -> (f64, f64, f64, f64, f64) {
    let t43357 = t1105 * t1109;
    let t43373 = t8589 * t3886;
    let t43375 = t829 * t830 * t43373;
    let t43451 = t376 * t3772;
    let t43466 = t13173 * t2366 * t833;
    let t43487 = t4414 * t13207;
    (t43357, t43375, t43451, t43466, t43487)
}
