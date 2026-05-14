//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 785/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk785<F: Float>(t572: F, t7148: F, t2735: F, t185: F, t1019: F, t1680: F, t1791: F, t2722: F, t661: F, t1621: F, t639: F, t1724: F, t2601: F, t2598: F, t4913: F, t172: F, t2824: F) -> (F, F, F, F, F, F) {
    let t7153 = t7148 * t572;
    let t7154 = t2735 * t7153;
    let t7156 = 8.0 / 45.0 * t185 * t7154;
    let t7158 = 4.0 / 15.0 * t1680 * t1019;
    let t7159 = t1791 * t2722;
    let t7160 = t7159 * t661;
    let t7161 = t1621 * t7160;
    let t7163 = 8.0 / 15.0 * t639 * t7161;
    let t7164 = t2601 * t1724;
    let t7165 = t1621 * t7164;
    let t7167 = 4.0 / 15.0 * t639 * t7165;
    let t7169 = 8.0 / 15.0 * t4913 * t2598;
    let t7170 = t172 * t2824;
    (t7156, t7158, t7163, t7167, t7169, t7170)
}
