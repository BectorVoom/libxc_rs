//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 849/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk849<F: Float>(t5106: F, t5117: F, t5127: F, t5131: F, t5139: F, t7138: F, t7140: F, t7145: F, t7147: F, t7152: F, t7156: F, t7158: F, t7163: F, t7167: F, t7169: F, t7173: F, t7175: F) -> (F, F, F, F, F, F) {
    let t7176 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5106;
    let t7177 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t5117;
    let t7178 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t5127;
    let t7179 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t5131;
    let t7180 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t5139;
    let t7181 = t7138 + t7140 - t7145 + t7147 - t7152 + t7156 - t7158 + t7163 + t7167 - t7169 + t7173 + t7175 + t7176 - t7177 + t7178 - t7179 - t7180;
    (t7176, t7177, t7178, t7179, t7180, t7181)
}
