//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 878/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk878<F: Float>(t254: F, t6: F, t6469: F, t2323: F, t3268: F, t1113: F, t904: F) -> (F, F, F) {
    let t9482 = t254 * t6 * t6469;
    let t9498 = F::new(7.0) / F::new(576.0) * t2323 * t3268;
    let t9499 = t904 * t1113;
    (t9482, t9498, t9499)
}
