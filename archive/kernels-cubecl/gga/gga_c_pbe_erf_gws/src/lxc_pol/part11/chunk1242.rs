//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1242/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1242<F: Float>(t45283: F, t37507: F, t37363: F, t37377: F, t49658: F, t49660: F, t49661: F, t49663: F, t49664: F, t49667: F, t49671: F, t45351: F) -> (F, F, F, F) {
    let t49672 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t45283;
    let t49673 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t37507;
    let t49674 = t49658 - t49660 + t49661 - t49663 - t49664 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t37363 - F::cast_from(119.0_f64) / F::cast_from(144.0_f64) * t37377 - t49667 - t49671 - t49672 + t49673;
    let t49681 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t45351;
    (t49672, t49673, t49674, t49681)
}
