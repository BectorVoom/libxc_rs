//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 987/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk987(t5: f64, t12722: f64, t112: f64, t111: f64, t4025: f64, t1441: f64, t2319: f64, t649: f64, t671: f64, t2363: f64, t88: f64, t1454: f64, t2281: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t12723 = piecewise3(t8, 0.0_f64, t12722);
    let t12724 = t12723 * t112;
    let t12725 = t4025 * t111;
    let t12728 = t1441 * t2319;
    let t12734 = t649 * t671;
    let t12739 = t88 * t2363;
    let t12747 = t2281 * t1454;
    (t12724, t12725, t12728, t12734, t12739, t12747)
}
