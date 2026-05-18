//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1338/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1338<F: Float>(t2409: F, t26668: F, t3965: F, t14001: F, t14466: F, t3959: F, t9328: F, t26655: F, t14765: F, t3074: F, t4395: F, t2362: F) -> (F, F, F, F, F) {
    let t54564 = t3965 * t2409 * t26668;
    let t54566 = t14001 * t14466;
    let t54567 = F::new(7.0) / F::new(72.0) * t54566;
    let t54572 = t3959 * t9328;
    let t54575 = t3965 * t2409 * t26655;
    let t54580 = t3074 * t4395 * t14765;
    let t54581 = t54580 * t2362;
    (t54564, t54567, t54572, t54575, t54581)
}
