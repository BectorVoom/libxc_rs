//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 985/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk985<F: Float>(t18224: F, t47809: F, t47810: F, t47811: F, t47812: F, t47814: F, t47818: F, t47820: F, t47822: F, t47825: F, t47828: F, t12817: F, t22917: F, t5211: F, t41095: F, t950: F) -> (F, F, F) {
    let t47829 = t47809 + t47810 - t47811 - t47812 + t18224 + t47814 + t47818 + t47820 + t47822 - t47825 - t47828;
    let t47832 = 32.0 / 9.0 * t5211 * t22917 * t12817;
    let t47833 = t41095 * t950;
    (t47829, t47832, t47833)
}
