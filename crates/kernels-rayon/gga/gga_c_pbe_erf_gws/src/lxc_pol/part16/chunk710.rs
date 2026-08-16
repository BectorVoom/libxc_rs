//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 710/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk710(t3972: f64, t4138: f64, t1118: f64, t1178: f64, t371: f64, t3983: f64, t1134: f64, t3990: f64, t3991: f64, t3989: f64, t1162: f64, t1177: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4139 = t3972 * t4138;
    let t4141 = t1178 * t1118;
    let t4142 = t371 * t4141;
    let t4143 = t3983 * t4142;
    let t4146 = t3990 * t3991 * t1134;
    let t4147 = t3989 * t4146;
    let t4149 = t1178 * t1162;
    let t4150 = t371 * t4149;
    let t4151 = t1177 * t4150;
    (t4139, t4141, t4142, t4143, t4146, t4147, t4149, t4150, t4151)
}
