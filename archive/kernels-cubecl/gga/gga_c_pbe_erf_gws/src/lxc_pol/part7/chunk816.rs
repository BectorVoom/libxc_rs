//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 816/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk816<F: Float>(t6618: F, t6623: F, t6625: F, t6650: F, t6654: F, t6663: F, t6676: F, t6682: F, t6692: F, t6696: F, t6700: F, t6704: F, t6709: F, t6713: F) -> F {
    let t6736 = t6618 + t6623 - t6625 + t6650 + t6654 - t6663 + t6676 + t6682 + t6692 - t6696 - t6700 - t6704 - t6709 - t6713;
    t6736
}
