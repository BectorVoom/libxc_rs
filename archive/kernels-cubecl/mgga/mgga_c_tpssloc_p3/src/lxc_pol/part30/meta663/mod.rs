//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta663 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2086;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta663<F: Float>(t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F, t26418: F, t7736: F, t80854: F, t81064: F, t22704: F, t26410: F, t26432: F, t6897: F, t794: F, t22642: F, t22690: F, t26395: F, t22863: F, t7737: F, t26448: F, t90497: F, t215: F, t6916: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90957, t90962, t90964, t90971, t90980, t90983) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2086::<F>(t26462, t6914, t22705, t26414, t81228, t26415, t81159, t26418, t7736, t80854, t81064, t22704, t26410);
        let (t90984, t90988, t90993, t91000, t91002, t91004) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2087::<F>(t90983, t26432, t6897, t794, t22642, t22690, t26395, t22863, t7737, t26448, t90497, t215, t6916);
    (t90957, t90962, t90964, t90971, t90980, t90984, t90988, t90993, t91000, t91002, t91004)
}
