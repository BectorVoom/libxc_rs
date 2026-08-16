//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1145/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1145(t14138: f64, t14733: f64, t1173: f64, t3202: f64, t14001: f64, t4130: f64, t13953: f64, t4135: f64, t3294: f64, t3975: f64, t3972: f64, t1112: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14734 = t14733 * t14138;
    let t14737 = t1173 * t3202;
    let t14745 = t14001 * t4130;
    let t14752 = t13953 * t4135;
    let t14754 = t3975 * t3294;
    let t14755 = t3972 * t14754;
    let t14765 = t1112 * t331;
    (t14734, t14737, t14745, t14752, t14754, t14755, t14765)
}
