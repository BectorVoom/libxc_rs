//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 786/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk786<F: Float>(t184: F, t7170: F, t564: F, t1872: F, t2790: F, t5106: F, t5117: F, t5127: F, t5131: F, t5139: F, t7138: F, t7140: F, t7145: F, t7147: F, t7152: F, t7156: F, t7158: F, t7163: F, t7167: F, t7169: F) -> (F, F, F, F, F, F, F, F) {
    let t7171 = t7170 * t184;
    let t7173 = 8.0 / 15.0 * t7171 * t564;
    let t7175 = 4.0 / 15.0 * t2790 * t1872;
    let t7176 = 8.0 / 45.0 * t5106;
    let t7177 = 16.0 / 135.0 * t5117;
    let t7178 = 32.0 / 135.0 * t5127;
    let t7179 = 16.0 / 135.0 * t5131;
    let t7180 = 16.0 / 135.0 * t5139;
    let t7181 = t7138 + t7140 - t7145 + t7147 - t7152 + t7156 - t7158 + t7163 + t7167 - t7169 + t7173 + t7175 + t7176 - t7177 + t7178 - t7179 - t7180;
    (t7173, t7175, t7176, t7177, t7178, t7179, t7180, t7181)
}
