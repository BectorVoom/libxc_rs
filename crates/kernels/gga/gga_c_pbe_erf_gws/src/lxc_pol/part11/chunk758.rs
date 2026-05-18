//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 758/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk758<F: Float>(t12422: F, t142: F, t10208: F, t12381: F, t12407: F, t12413: F, t169: F, t299: F, t301: F, t5617: F, t8277: F, t8281: F, t8296: F, t8310: F, t8314: F, t8497: F, t988: F) -> (F, F) {
    let t12423 = t12422 * t142;
    let t12425 = -F::new(0.1743404491073215162e-2) * t8277 + F::new(0.40679438125041687114e-2) * t8281 + F::new(2.0) * t988 * t12407 - F::new(0.15965645347006145458e0) * t8296 - F::new(0.35922702030763827282e-1) * t10208 - t5617 - F::new(9.0) * t8497 * t12413 - F::new(0.54655730795145295329e-4) * t8310 + F::new(0.59450495276030562782e0) * t8314 + F::new(0.20267214298646782767e-1) * t169 * t299 * t12381 * t301 + t988 * t12423;
    (t12423, t12425)
}
