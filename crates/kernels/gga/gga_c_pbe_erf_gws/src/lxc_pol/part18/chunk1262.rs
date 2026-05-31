//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1262/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1262<F: Float>(t353: F, t4183: F, t4386: F, t810: F, t14001: F, t14466: F, t14765: F, t3074: F, t4395: F, t1161: F, t874: F, t3102: F, t859: F) -> (F, F, F, F, F) {
    let t54550 = t4386 * t353 * t4183 * t810;
    let t54566 = t14001 * t14466;
    let t54567 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54566;
    let t54580 = t3074 * t4395 * t14765;
    let t54590 = t1161 * t874;
    let t54595 = t859 * t3102;
    (t54550, t54567, t54580, t54590, t54595)
}
