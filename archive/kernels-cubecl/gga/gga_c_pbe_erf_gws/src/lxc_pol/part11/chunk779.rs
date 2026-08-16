//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 779/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk779<F: Float>(t12659: F, t1885: F, t1820: F, t12611: F, t12615: F, t12619: F, t12622: F, t12625: F, t12629: F, t12633: F, t12637: F, t12641: F, t12645: F, t12649: F, t12653: F, t12655: F, t12656: F, t12658: F, t5929: F) -> (F, F, F) {
    let t12660 = t1885 * t12659;
    let t12662 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1820 * t12660;
    let t12663 = -t12611 + t12615 - t12619 - t12622 + t12625 + t12629 + t12633 - t12637 + t12641 + t12645 + t12649 + t12653 - t12655 - t12656 - t12658 - t12662 + t5929;
    (t12660, t12662, t12663)
}
