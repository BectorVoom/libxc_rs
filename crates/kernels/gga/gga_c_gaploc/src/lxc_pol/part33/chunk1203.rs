//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1203/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1203<F: Float>(t30105: F, t31922: F, t31924: F, t31928: F, t31930: F, t31932: F, t31935: F, t31939: F, t31942: F, t31945: F, t31948: F, t31952: F, t31956: F, t31958: F, t30113: F, t30118: F, t30120: F, t30123: F, t31961: F, t31966: F, t31969: F, t31974: F, t31984: F, t31988: F, t31990: F, t31994: F, t31998: F, t32001: F, t32003: F) -> (F, F) {
    let t38384 = -t30105 + t31922 - t31924 - t31928 + t31930 - t31932 - t31935 - t31939 + t31942 + t31945 - t31948 - t31952 - t31956 + t31958;
    let t38385 = t31961 + t31966 + t31969 + t31974 + t31984 + t31988 - t31990 - t31994 - t31998 + t32001 + t32003 - t30113 + t30118 + t30120 + t30123;
    (t38384, t38385)
}
