//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1201/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1201(t19703: f64, t3632: f64, t1381: f64, t17960: f64, t17964: f64, t3638: f64, t3667: f64, t5552: f64, t3671: f64, t3678: f64, t1385: f64, t17974: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19704 = t19703 * t3632;
    let t19706 = t17960 * t1381;
    let t19708 = t17964 * t3638;
    let t19710 = t5552 * t3667;
    let t19712 = t17964 * t3671;
    let t19716 = t17964 * t3678;
    let t19718 = t17974 * t1385;
    (t19704, t19706, t19708, t19710, t19712, t19716, t19718)
}
