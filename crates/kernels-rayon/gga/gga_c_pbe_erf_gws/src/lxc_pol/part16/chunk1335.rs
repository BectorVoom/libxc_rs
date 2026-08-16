//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1335/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1335(t54135: f64, t54152: f64, t51252: f64, t54133: f64, t54137: f64, t54139: f64, t54142: f64, t54144: f64, t54146: f64, t54148: f64, t54150: f64, t54154: f64) -> f64 {
    let t55491 = 7.0_f64 / 72.0_f64 * t54135;
    let t55500 = 7.0_f64 / 72.0_f64 * t54152;
    let t55502 = t54133 / 8.0_f64 - t55491 + t54137 / 128.0_f64 + 3.0_f64 / 128.0_f64 * t54139 - 7.0_f64 / 144.0_f64 * t51252 + t54142 / 48.0_f64 - t54144 / 192.0_f64 - t54146 / 48.0_f64 + t54148 / 24.0_f64 - t54150 / 48.0_f64 + t55500 - t54154 / 192.0_f64;
    t55502
}
