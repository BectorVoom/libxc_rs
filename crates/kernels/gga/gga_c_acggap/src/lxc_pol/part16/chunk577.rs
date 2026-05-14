//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 577/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk577<F: Float>(t4804: F, t4856: F, t4857: F, t5639: F, t5643: F, t5647: F, t5649: F, t5653: F, t5657: F, t5661: F, t5664: F, t3107: F, t3110: F, t3112: F, t3128: F, t3142: F, t3144: F, t3161: F, t4812: F, t4814: F, t4860: F, t4863: F) -> (F, F) {
    let t5777 = -2.0 / 3.0 * t5639 - 3.0 / 2.0 * t5643 + t5647 + t5649 / 3.0 + t5653 / 2.0 + t5657 / 12.0 - t5661 / 24.0 + t4856 + t4857 - t5664 / 4.0 + t4804 / 3.0;
    let t5783 = -t4860 - t4812 / 6.0 - 14.0 / 9.0 * t4814 - t4863 + t3107 - t3110 + t3112 / 6.0 - t3128 / 12.0 - t3142 - 7.0 / 9.0 * t3144 + t3161;
    (t5777, t5783)
}
