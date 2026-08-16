//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2681/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2681(t12178: f64, t12255: f64, t12303: f64, t12371: f64, t16305: f64, t16311: f64, t16312: f64, t19735: f64, t19876: f64, t3803: f64, t3805: f64, t3807: f64, t40168: f64, t40285: f64, t40293: f64, t40295: f64, t5246: f64, t5301: f64, t54258: f64, t54585: f64, t54591: f64, t54607: f64, t54609: f64, t54612: f64, t54614: f64) -> f64 {
    let t54625 = 7.0_f64 / 1536.0_f64 * t54585 - t5246 * t16305 * t19735 * t16312 / 64.0_f64 - t5246 * t16305 * t16311 * t54591 / 128.0_f64 + t3803 * t16305 * t54258 * t3807 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t40285 - 119.0_f64 / 576.0_f64 * t40293 + 7.0_f64 / 1152.0_f64 * t40295 - t5246 * t3805 * t5301 * t12255 / 128.0_f64 - 7.0_f64 / 384.0_f64 * t54607 - 7.0_f64 / 1536.0_f64 * t54609 + t54612 - 15.0_f64 / 128.0_f64 * t54614 * t40168 * t5301 * t12303 - t19876 * t12371 / 128.0_f64 + t3803 * t3805 * t5301 * t12178 / 768.0_f64;
    t54625
}
