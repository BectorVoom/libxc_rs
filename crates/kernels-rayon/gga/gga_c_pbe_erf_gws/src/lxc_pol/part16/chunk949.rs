//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 949/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk949(t5906: f64, t5910: f64, t5912: f64, t7140: f64, t7145: f64, t7147: f64, t7152: f64, t7156: f64, t7158: f64, t7163: f64, t7167: f64, t7169: f64, t7173: f64, t7175: f64, t7176: f64, t7177: f64, t7178: f64) -> f64 {
    let t8421 = t7140 - t7145 + t7147 - t7152 + t7156 - t7158 + t7163 + t7167 - t7169 + t5906 + 4.0_f64 / 3.0_f64 * t5910 + 8.0_f64 / 3.0_f64 * t5912 + t7173 + t7175 + t7176 - t7177 + t7178;
    t8421
}
