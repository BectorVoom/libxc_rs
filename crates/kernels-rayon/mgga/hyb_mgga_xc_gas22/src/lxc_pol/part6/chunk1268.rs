//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1268/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1268(t10: f64, t17: f64, t23323: f64, t3021: f64, t23329: f64, t3: f64, t19735: f64, t19737: f64, t19739: f64, t19744: f64, t23253: f64, t23255: f64, t23257: f64, t23284: f64, t23311: f64, t23328: f64, t23335: f64, t23355: f64) -> (f64, f64) {
    let t27275 = t23323 * t10 * t3021 * t17;
    let t27276 = t23329 * t3;
    let t27288 = 28.0_f64 / 729.0_f64 * t19735 - 2.0_f64 / 243.0_f64 * t19737 - 4.0_f64 / 729.0_f64 * t19739 + 4.0_f64 / 243.0_f64 * t19744 + 2.0_f64 / 81.0_f64 * t23253 - 4.0_f64 / 81.0_f64 * t23255 + 2.0_f64 / 27.0_f64 * t23257 - 40.0_f64 / 243.0_f64 * t27275 * t23328 * t27276 + 16.0_f64 / 27.0_f64 * t27275 * t23335 * t27276 - 8.0_f64 / 9.0_f64 * t27275 * t23355 * t27276 - 16.0_f64 / 729.0_f64 * t23284 + 2.0_f64 / 243.0_f64 * t23311;
    (t27275, t27288)
}
