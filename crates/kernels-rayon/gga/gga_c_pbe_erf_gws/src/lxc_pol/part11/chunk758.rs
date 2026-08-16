//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 758/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk758(t12422: f64, t142: f64, t10208: f64, t12381: f64, t12407: f64, t12413: f64, t169: f64, t299: f64, t301: f64, t5617: f64, t8277: f64, t8281: f64, t8296: f64, t8310: f64, t8314: f64, t8497: f64, t988: f64) -> (f64, f64) {
    let t12423 = t12422 * t142;
    let t12425 = -0.1743404491073215162e-2_f64 * t8277 + 0.40679438125041687114e-2_f64 * t8281 + 2.0_f64 * t988 * t12407 - 0.15965645347006145458e0_f64 * t8296 - 0.35922702030763827282e-1_f64 * t10208 - t5617 - 9.0_f64 * t8497 * t12413 - 0.54655730795145295329e-4_f64 * t8310 + 0.59450495276030562782e0_f64 * t8314 + 0.20267214298646782767e-1_f64 * t169 * t299 * t12381 * t301 + t988 * t12423;
    (t12423, t12425)
}
