//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 756/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk756(t10231: f64, t10239: f64, t10245: f64, t12324: f64, t12381: f64, t145: f64, t169: f64, t242: f64, t5700: f64, t5707: f64, t5717: f64, t5730: f64, t5732: f64, t8347: f64, t8357: f64, t8363: f64, t8373: f64) -> f64 {
    let t12384 = t5700 - 0.42447554366239164361e0_f64 * t8363 - t5707 + 0.15917832887339686635e0_f64 * t10231 + 0.3183566577467937327e0_f64 * t8357 + t5717 - 0.31835665774679373271e-1_f64 * t169 * t12324 * t242 - 0.95506997324038119813e-1_f64 * t10239 - 0.95506997324038119813e-1_f64 * t8373 - t5730 - t5732 + 0.9598512193592288454e0_f64 * t8347 - 0.3199504064530762818e0_f64 * t10245 + 0.533250677421793803e-1_f64 * t145 * t12381;
    t12384
}
