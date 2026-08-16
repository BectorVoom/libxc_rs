//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1304/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1304(t10218: f64, t10221: f64, t10226: f64, t10229: f64, t10234: f64, t10237: f64, t10242: f64, t10245: f64, t10250: f64, t10253: f64, t10258: f64, t10262: f64, t2070: f64, t3165: f64, t3191: f64, t3196: f64, t3211: f64, t3216: f64, t3221: f64, t8335: f64) -> f64 {
    let t28414 = t10218 * t2070 / 258048.0_f64 + t10221 * t2070 / 491520.0_f64 - t3211 * t8335 / 3440640.0_f64 - t10226 * t2070 / 6881280.0_f64 - t10229 * t2070 / 13271040.0_f64 + t3216 * t8335 / 0.10616832e9_f64 + t10234 * t2070 / 0.21233664e9_f64 + t10237 * t2070 / 412876800.0_f64 - t3221 * t8335 / 0.37158912e10_f64 - t10242 * t2070 / 0.74317824e10_f64 - 2.0_f64 / 3.0_f64 * t10245 * t2070 + t3165 * t8335 / 3.0_f64 + t10250 * t2070 / 6.0_f64 + t10253 * t2070 / 8.0_f64 - t3191 * t8335 / 24.0_f64 - t10258 * t2070 / 48.0_f64 - t10262 * t2070 / 80.0_f64 + t3196 * t8335 / 320.0_f64;
    t28414
}
