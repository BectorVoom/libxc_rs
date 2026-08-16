//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1339/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1339(t54236: f64, t54238: f64, t54257: f64, t54259: f64, t51341: f64, t51358: f64, t54241: f64, t54246: f64, t54248: f64, t54251: f64, t54255: f64, t54261: f64) -> f64 {
    let t55547 = 7.0_f64 / 72.0_f64 * t54236;
    let t55548 = 7.0_f64 / 144.0_f64 * t54238;
    let t55556 = 7.0_f64 / 72.0_f64 * t54257;
    let t55557 = 7.0_f64 / 36.0_f64 * t54259;
    let t55559 = t55547 - t55548 - 7.0_f64 / 36.0_f64 * t51341 + t54241 / 24.0_f64 + t54246 / 12.0_f64 + t54248 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t51358 - t54251 / 8.0_f64 - t54255 / 24.0_f64 + t55556 - t55557 - t54261 / 384.0_f64;
    t55559
}
