//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 997/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk997<F: Float>(t3139: F, t6646: F, t14101: F, t14055: F, t14060: F, t14061: F, t14065: F, t14067: F, t14070: F, t14073: F, t14074: F, t14076: F, t14081: F, t14085: F, t14086: F, t14088: F, t14094: F, t14097: F) -> (F, F) {
    let t14102 = t3139 * t6646;
    let t14103 = t14101 * t14102;
    let t14105 = 5.0 / 384.0 * t14055 + t14060 - t14061 / 384.0 - t14065 / 24.0 + t14067 / 384.0 - t14070 / 48.0 + t14073 - t14074 / 768.0 - t14076 / 768.0 + t14081 + t14085 + t14086 / 768.0 + t14088 / 768.0 - t14094 / 96.0 - t14097 / 96.0 + t14103 / 48.0;
    (t14102, t14105)
}
