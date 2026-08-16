//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1117/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1117<F: Float>(t12556: F, t2615: F, t12634: F, t5218: F, t7495: F, t12804: F, t24848: F, t18224: F, t47809: F, t47810: F, t47811: F, t47812: F, t47814: F, t47818: F, t47820: F) -> (F, F, F, F) {
    let t47822 = F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t2615 * t12556;
    let t47825 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t5218 * t7495 * t12634;
    let t47828 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5218 * t24848 * t12804;
    let t47829 = t47809 + t47810 - t47811 - t47812 + t18224 + t47814 + t47818 + t47820 + t47822 - t47825 - t47828;
    (t47822, t47825, t47828, t47829)
}
