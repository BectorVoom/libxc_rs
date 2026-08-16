//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1074/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1074(t4051: f64, t6359: f64, t180: f64, t4046: f64, t6383: f64, t2124: f64, t2132: f64, t6394: f64, t1270: f64, t181: f64, t178: f64, t10350: f64, t173: f64, t3227: f64, t3245: f64, t3246: f64, t3252: f64, t3255: f64, t3258: f64, t4052: f64, t747: f64, t751: f64, t8373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10364 = t6359 * t4051;
    let t10373 = t180 * t4046;
    let t10394 = t6383 * t4051;
    let t10397 = t2124 * t4046;
    let t10403 = t2132 * t4046;
    let t10408 = t6394 * t4051;
    let t10411 = t1270 * t181;
    let t10414 = t178 * t1270;
    let t10424 = 15.0_f64 / 2.0_f64 * t4052 * t3246 - 4.0_f64 * t3245 * t8373 - 5.0_f64 / 2.0_f64 * t10394 * t3246 - 2.0_f64 * t10397 * t3246 + t747 * t10350 * t180 / 2.0_f64 + t10403 * t3246 / 4.0_f64 + t3252 * t8373 / 2.0_f64 + t10408 * t3246 / 8.0_f64 - 8.0_f64 * t10411 * t3227 - 2.0_f64 * t10414 * t8373 - 4.0_f64 * t3255 * t4046 - t3258 * t10373 - 4.0_f64 * t751 * t10350 - t173 * t10350 * t180;
    (t10364, t10373, t10394, t10397, t10403, t10408, t10411, t10414, t10424)
}
