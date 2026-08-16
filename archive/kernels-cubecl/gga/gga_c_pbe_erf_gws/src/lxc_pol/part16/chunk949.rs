//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 949/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk949<F: Float>(t5906: F, t5910: F, t5912: F, t7140: F, t7145: F, t7147: F, t7152: F, t7156: F, t7158: F, t7163: F, t7167: F, t7169: F, t7173: F, t7175: F, t7176: F, t7177: F, t7178: F) -> F {
    let t8421 = t7140 - t7145 + t7147 - t7152 + t7156 - t7158 + t7163 + t7167 - t7169 + t5906 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5910 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5912 + t7173 + t7175 + t7176 - t7177 + t7178;
    t8421
}
