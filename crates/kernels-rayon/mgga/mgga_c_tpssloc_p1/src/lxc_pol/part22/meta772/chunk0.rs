//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2632/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2632(t3447: f64, t4904: f64, t64779: f64, t15402: f64, t21749: f64, t15376: f64, t15382: f64, t15390: f64, t15395: f64, t18543: f64, t18546: f64, t44635: f64, t458: f64, t4900: f64, t4919: f64, t4936: f64, t52100: f64, t52368: f64, t6138: f64, t64644: f64, t64870: f64, t65018: f64, t65056: f64, t71193: f64, t72688: f64) -> (f64, f64, f64) {
    let t73535 = t3447 * t64779 * t4904;
    let t73541 = t3447 * t15402 * t21749;
    let t73571 = -0.24999999999999999999e-2_f64 * t3447 * t458 * t6138 * t4936 + 0.66666666666666666663e-2_f64 * t3447 * t4900 * t71193 + t52368 - 0.25925925925925925926e-2_f64 * t3447 * t15395 * t72688 + 0.25925925925925925926e-2_f64 * t3447 * t52100 * t65018 - 0.11111111111111111111e-2_f64 * t3447 * t64644 * t15382 - 0.11111111111111111111e-2_f64 * t3447 * t15390 * t65056 - 0.44444444444444444443e-2_f64 * t15376 * t18543 - 0.88888888888888888886e-2_f64 * t15376 * t18546 + 0.33333333333333333332e-2_f64 * t3447 * t4919 * t64870 - 0.10288065843621399177e-3_f64 * t44635;
    (t73535, t73541, t73571)
}
