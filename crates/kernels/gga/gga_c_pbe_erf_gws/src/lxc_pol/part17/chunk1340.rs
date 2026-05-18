//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1340/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1340<F: Float>(t1113: F, t28947: F, t3972: F, t3975: F, t1161: F, t874: F, t13776: F, t2171: F, t50956: F, t3102: F, t859: F, t13792: F) -> (F, F, F) {
    let t54588 = t3972 * t3975 * t1113 * t28947;
    let t54590 = t1161 * t874;
    let t54593 = t13776 * t50956 * t54590 * t2171;
    let t54595 = t859 * t3102;
    let t54596 = t13792 * t54595;
    (t54588, t54593, t54596)
}
