//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1175/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1175<F: Float>(t2200: F, t863: F, t864: F, t2173: F, t2157: F, t6439: F, t2135: F, t3138: F, t3139: F, t18424: F, t18428: F, t18432: F, t18435: F, t18439: F, t18441: F, t18445: F, t18448: F, t18452: F, t18456: F, t18460: F, t18462: F, t19472: F) -> (F, F, F) {
    let t20962 = t863 * t864 * t2200;
    let t20963 = t20962 * t2173;
    let t20964 = F::new(35.0) / F::new(18.0) * t20963;
    let t20965 = t2157 * t6439;
    let t20969 = t3138 * t3139 * t2135 * t20965 / F::new(12.0);
    let t20974 = -t19472 + t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462;
    (t20964, t20969, t20974)
}
