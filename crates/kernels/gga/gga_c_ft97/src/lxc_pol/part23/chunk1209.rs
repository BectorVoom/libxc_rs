//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1209/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1209<F: Float>(t121897: F, t24438: F, t6118: F, t24437: F, t2574: F, t5053: F, t6119: F, t747: F, t10157: F, t18641: F, t110041: F, t110042: F, t110043: F, t110044: F, t110060: F, t122682: F, t122686: F, t122689: F) -> (F, F, F, F) {
    let t122692 = t6118 * t24438 * t121897;
    let t122697 = t24437 * t2574 * t6119 * t5053 * t747;
    let t122701 = t6118 * t10157 * t6119 * t18641;
    let t122702 = t110041 + t110042 - t110043 + t110044 - t122682 / 54.0 + t122686 / 3.0 - t122689 / 9.0 - t122692 / 9.0 + t110060 - t122697 / 6.0 - t122701;
    (t122692, t122697, t122701, t122702)
}
