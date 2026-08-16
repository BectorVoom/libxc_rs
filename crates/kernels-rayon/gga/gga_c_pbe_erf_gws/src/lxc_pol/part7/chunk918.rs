//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 918/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk918(t17231: f64, t1680: f64, t1806: f64, t1820: f64, t4887: f64, t5125: f64, t5126: f64, t5312: f64, t17205: f64, t17208: f64, t17211: f64, t17215: f64, t17219: f64, t17222: f64, t17225: f64, t17229: f64) -> (f64, f64, f64, f64, f64) {
    let t17232 = 16.0_f64 / 45.0_f64 * t17231;
    let t17234 = 16.0_f64 / 5.0_f64 * t1680 * t1806;
    let t17236 = t1820 * t5125 * t4887;
    let t17237 = 64.0_f64 / 45.0_f64 * t17236;
    let t17238 = t5312 * t5126;
    let t17239 = 128.0_f64 / 45.0_f64 * t17238;
    let t17240 = t17205 + t17208 + t17211 + t17215 - t17219 + t17222 - t17225 + t17229 - t17232 + t17234 + t17237 + t17239;
    (t17232, t17234, t17237, t17239, t17240)
}
