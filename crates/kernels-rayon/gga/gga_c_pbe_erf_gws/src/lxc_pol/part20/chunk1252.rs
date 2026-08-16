//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1252/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1252(t54052: f64, t2209: f64, t4026: f64, t863: f64, t1135: f64, t9246: f64, t2134: f64, t28139: f64, t850: f64, t3065: f64, t3167: f64, t3253: f64, t51255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54053 = 7.0_f64 / 192.0_f64 * t54052;
    let t54055 = t863 * t4026 * t2209;
    let t54071 = t9246 * t1135;
    let t54072 = t2134 * t54071;
    let t54073 = 7.0_f64 / 144.0_f64 * t54072;
    let t54079 = t850 * t28139;
    let t54084 = t3065 * t3167;
    let t54087 = t51255 * t3253;
    (t54053, t54055, t54071, t54073, t54079, t54084, t54087)
}
