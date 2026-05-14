//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1131/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1131<F: Float>(t2409: F, t26655: F, t3965: F, t14765: F, t3074: F, t4395: F, t2362: F, t1113: F, t28947: F, t3972: F, t3975: F, t1161: F, t874: F, t13776: F, t2171: F, t50956: F) -> (F, F, F, F) {
    let t54575 = t3965 * t2409 * t26655;
    let t54580 = t3074 * t4395 * t14765;
    let t54581 = t54580 * t2362;
    let t54588 = t3972 * t3975 * t1113 * t28947;
    let t54590 = t1161 * t874;
    let t54593 = t13776 * t50956 * t54590 * t2171;
    (t54575, t54581, t54588, t54593)
}
