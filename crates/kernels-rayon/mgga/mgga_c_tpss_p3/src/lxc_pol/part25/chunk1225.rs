//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1225/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1225(t1830: f64, t3537: f64, t6399: f64, t645: f64, t20319: f64, t485: f64, t1600: f64, t5815: f64, t1163: f64, t6323: f64, t1846: f64, t19577: f64, t2056: f64, t3493: f64, t3499: f64, t5816: f64, t5820: f64, t5937: f64, t6103: f64, t624: f64, t6243: f64, t626: f64, t6318: f64, t6324: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20368 = t1830 * t3537;
    let t20371 = t6399 * t645;
    let t20374 = t485 * t20319;
    let t20379 = t1600 * t5815;
    let t20386 = t1163 * t6323;
    let t20395 = t1846 * t19577 - 2.0_f64 * t20368 * t626 - 2.0_f64 * t20371 * t626 - 2.0_f64 * t20374 * t626 - 2.0_f64 * t20379 * t626 - 2.0_f64 * t20386 * t626 - 2.0_f64 * t2056 * t6318 - 2.0_f64 * t2056 * t6324 - 2.0_f64 * t3493 * t5820 - 2.0_f64 * t3499 * t6318 - 2.0_f64 * t3499 * t6324 - 2.0_f64 * t5816 * t6103 + t5937 * t6243 - t624 * t6399;
    (t20368, t20371, t20374, t20379, t20386, t20395)
}
