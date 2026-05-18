//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 740/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk740<F: Float>(t4688: F, t4711: F, t4714: F, t4718: F, t4799: F, t4803: F, t4807: F, t4811: F, t4815: F, t4818: F, t4820: F, t4822: F, t4824: F, t4826: F) -> F {
    let t6079 = -t4799 - t4803 + t4807 + t4811 - t4815 + t4688 + t4711 - t4714 - t4718 - t4818 + t4820 - t4822 + t4824 + t4826;
    t6079
}
