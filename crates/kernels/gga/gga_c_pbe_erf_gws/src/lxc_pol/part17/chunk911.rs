//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 911/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk911<F: Float>(t7694: F, t7962: F, t1820: F, t2575: F, t4934: F, t1620: F, t2826: F, t583: F, t5564: F, t5562: F, t7927: F, t7931: F, t7934: F, t7939: F, t7943: F, t7944: F, t7947: F, t7949: F, t7953: F, t7955: F, t7958: F, t7961: F) -> (F, F, F, F, F) {
    let t7963 = t7694 * t7962;
    let t7965 = F::new(16.0) / F::new(45.0) * t1820 * t7963;
    let t7966 = t4934 * t2575;
    let t7968 = F::new(32.0) / F::new(135.0) * t1620 * t7966;
    let t7970 = F::new(8.0) / F::new(45.0) * t2826 * t583;
    let t7971 = F::new(8.0) / F::new(45.0) * t5564;
    let t7972 = t7927 - t7931 + t7934 - t7939 - t7943 - t7944 - t7947 + t5562 - t7949 + t7953 + t7955 - t7958 - t7961 + t7965 + t7968 + t7970 + t7971;
    (t7965, t7968, t7970, t7971, t7972)
}
