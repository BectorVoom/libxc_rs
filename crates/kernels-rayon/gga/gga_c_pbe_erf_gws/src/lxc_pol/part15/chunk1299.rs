//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1299/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1299(t1135: f64, t9246: f64, t2134: f64, t54043: f64, t54045: f64, t54048: f64, t54053: f64, t54057: f64, t54059: f64, t54061: f64, t54063: f64, t54065: f64, t54067: f64, t54069: f64) -> f64 {
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54073 = 7.0_f64 / 144.0_f64 * t54072;
    let t54074 = t54043 / 24.0_f64 + t54045 / 384.0_f64 + t54048 / 64.0_f64 - t54053 - t54057 / 8.0_f64 - 5.0_f64 / 192.0_f64 * t54059 + t54061 / 96.0_f64 + t54063 / 384.0_f64 - t54065 / 192.0_f64 + t54067 / 192.0_f64 - t54069 / 32.0_f64 + t54073;
    t54074
}
