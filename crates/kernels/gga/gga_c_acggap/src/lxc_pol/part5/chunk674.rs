//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 674/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk674<F: Float>(t5045: F, t2806: F, t2812: F, t2979: F, t2983: F, t2989: F, t2995: F, t5030: F, t5031: F, t5033: F, t5035: F, t5037: F, t5038: F, t5039: F, t5041: F, t5044: F) -> (F,) {
    let t5046 = 0.5848223622634646207e0 * t5045;
    let t5047 = t5030 - t2979 - t2983 - t5031 + t5033 + t5035 - t5037 - t2989 + t2806 - t2812 + t5038 + t2995 + t5039 - t5041 - t5044 - t5046;
    (t5047,)
}
