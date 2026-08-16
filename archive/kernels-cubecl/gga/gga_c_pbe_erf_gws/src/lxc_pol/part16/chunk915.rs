//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 915/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk915<F: Float>(t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t4744: F, t6918: F, t6932: F, t6966: F, t6969: F, t7984: F, t7985: F, t7987: F, t7989: F, t7991: F, t7992: F) -> F {
    let t7993 = t6918 + t4503 - t4506 - t4513 + t4539 + t4542 + t6932 + t6966 + t6969 - t7984 - t7985 + t7987 - t7989 + t7991 + t7992 + t4744;
    t7993
}
