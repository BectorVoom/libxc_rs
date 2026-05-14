//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 746/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk746<F: Float>(t10634: F, t12622: F, t12625: F, t12629: F, t12633: F, t12637: F, t12641: F, t12645: F, t12649: F, t12653: F, t12655: F, t12656: F, t12658: F, t12662: F, t12705: F, t12707: F, t12713: F, t7541: F) -> (F,) {
    let t13016 = -t12622 + t12625 + t12629 + t12633 - t12637 - 2.0 / 15.0 * t10634 + t12641 + t12645 + t12649 + t12653 - t12655 - 2.0 / 9.0 * t7541 - t12656 - t12658 - t12662 - t12705 + t12707 + t12713;
    (t13016,)
}
