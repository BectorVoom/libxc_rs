//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 903/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk903<F: Float>(t6659: F, t858: F, t8939: F, t884: F, t2079: F, t2112: F, t326: F, t3107: F, t860: F, t2119: F, t3039: F, t2124: F, t6333: F, t3128: F, t6258: F, t8923: F, t8925: F, t8927: F, t8930: F, t8932: F, t8936: F, t8938: F) -> (F, F, F, F, F, F, F) {
    let t8941 = t6659 * t858 * t8939;
    let t8943 = t884 * t8941 / 4.0;
    let t8944 = t2079 * t2112;
    let t8945 = t326 * t8944;
    let t8946 = t8945 * t3107;
    let t8948 = t8946 * t860 / 96.0;
    let t8949 = t3039 * t2119;
    let t8951 = t8949 * t2124 / 48.0;
    let t8952 = 7.0 / 72.0 * t6333;
    let t8954 = t3128 * t6258 / 48.0;
    let t8955 = t8923 - t8925 - t8927 - t8930 + t8932 + t8936 - t8938 - t8943 + t8948 - t8951 + t8952 - t8954;
    (t8943, t8945, t8948, t8951, t8952, t8954, t8955)
}
