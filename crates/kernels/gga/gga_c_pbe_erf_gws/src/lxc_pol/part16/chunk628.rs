//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 628/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk628<F: Float>(t532: F, t991: F, t159: F, t285: F, t143: F, t2873: F, t125: F, t1493: F, t1499: F, t1501: F, t169: F, t2033: F, t2035: F, t2037: F, t281: F, t2848: F, t2932: F, t2937: F, t2939: F, t296: F, t2984: F, t2986: F, t299: F, t2990: F, t301: F, t3011: F, t475: F, t988: F) -> (F, F) {
    let t3013 = t532 * t991;
    let t3015 = t3013 * t159 * t285;
    let t3021 = t143 * t2873;
    let t3024 = -F::new(0.11974234010254609094e-1) * t1493 - t1499 + t988 * t1501 - F::new(0.11974234010254609094e-1) * t281 * t2932 - F::new(0.11974234010254609094e-1) * t2937 + F::new(0.19957056683757681823e-1) * t2939 + t2984 * t125 + F::new(3.0) * t2986 * t2037 - t988 * t2033 + F::new(3.0) * t2035 * t2990 + t3011 * t296 - F::new(0.29056741517886919367e-3) * t3015 + F::new(0.20267214298646782767e-1) * t169 * t299 * t2848 * t301 + F::new(3.0) * t475 * t3021;
    (t3013, t3024)
}
