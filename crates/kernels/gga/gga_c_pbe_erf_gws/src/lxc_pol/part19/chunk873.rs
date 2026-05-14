//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 873/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk873<F: Float>(t10879: F, t10863: F, t10866: F, t10870: F, t10873: F, t10875: F, t10877: F, t5384: F, t5387: F, t5417: F, t7715: F, t7728: F, t7732: F, t7734: F, t7736: F, t7753: F, t7757: F) -> (F, F) {
    let t10880 = 8.0 / 45.0 * t10879;
    let t10882 = t10863 + t10866 - t7715 - t5384 + t5387 + t7728 + t7732 + 2.0 / 3.0 * t7734 + 0.2431111111111111111e0 * t7736 - t10870 - t10873 - t10875 - t7753 + t7757 + t10877 + t10880 + t5417 / 3.0;
    (t10880, t10882)
}
