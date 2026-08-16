//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1391/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1391(t10375: f64, t1942: f64, t1025: f64, t10346: f64, t1046: f64, t10485: f64, t10879: f64, t10886: f64, t10972: f64, t10998: f64, t1935: f64, t3043: f64, t3134: f64, t343: f64, t6717: f64, t6734: f64, t6765: f64, t83034: f64, t83038: f64, t83041: f64, t83043: f64, t83046: f64, t83054: f64, t83058: f64, t83061: f64, t83065: f64, t83068: f64, t83071: f64, t83075: f64) -> f64 {
    let t83080 = t1942 * t10375 / 5184.0_f64;
    let t83081 = -0.30279567070605293142e-3_f64 * t83034 + t6717 * t10998 / 48.0_f64 - t83038 * t1046 / 72.0_f64 + t83041 / 576.0_f64 + t83043 * t3134 / 256.0_f64 - t83046 / 72.0_f64 - 0.10093189023535097714e-3_f64 * t1935 * t10346 * t343 * t6734 + t83054 * t10485 / 256.0_f64 - t83058 * t10879 / 256.0_f64 - t83061 * t3043 / 512.0_f64 + t83065 * t10886 / 1536.0_f64 + t83068 * t1025 / 512.0_f64 + t83071 * t1046 / 768.0_f64 - 0.48447307312968469026e-2_f64 * t83075 + 5.0_f64 / 2592.0_f64 * t6765 * t10972 + t83080;
    t83081
}
