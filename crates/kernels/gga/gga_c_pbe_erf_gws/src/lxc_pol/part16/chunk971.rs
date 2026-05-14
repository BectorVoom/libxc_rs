//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 971/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk971<F: Float>(t13953: F, t3966: F, t1178: F, t2231: F, t371: F, t3983: F, t3979: F, t3997: F, t2412: F, t3959: F, t2409: F, t6787: F, t1176: F, t903: F, t923: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13954 = t13953 * t3966;
    let t13955 = 7.0 / 144.0 * t13954;
    let t13957 = t371 * t1178 * t2231;
    let t13958 = t3983 * t13957;
    let t13964 = t3979 * t3997;
    let t13965 = 7.0 / 2304.0 * t13964;
    let t13966 = t3959 * t2412;
    let t13968 = t2409 * t6787;
    let t13969 = t3959 * t13968;
    let t13972 = t1176 * t923 * t903;
    (t13954, t13955, t13957, t13958, t13964, t13965, t13966, t13968, t13969, t13972)
}
