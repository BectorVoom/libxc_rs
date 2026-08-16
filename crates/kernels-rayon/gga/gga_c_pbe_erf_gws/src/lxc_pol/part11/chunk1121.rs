//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1121/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1121(t18240: f64, t18243: f64, t18245: f64, t18261: f64, t18267: f64, t47862: f64, t47864: f64, t47866: f64, t47868: f64, t47870: f64, t47872: f64, t41208: f64) -> (f64, f64) {
    let t47873 = t47862 - t47864 + t47866 - t47868 + t18240 - t18243 - t18245 + t18261 + t18267 + t47870 + t47872;
    let t47874 = 128.0_f64 / 45.0_f64 * t41208;
    (t47873, t47874)
}
