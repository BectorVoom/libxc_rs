//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1396/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1396(t1412: f64, t6816: f64, t1353: f64, t1394: f64, t21969: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t22223: f64, t22229: f64, t22237: f64, t22240: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t6832: f64, t6837: f64, t6840: f64) -> f64 {
    let t22245 = t1412 * t6816;
    let t22246 = t22245 * t1353;
    let t22249 = t1394 * t21969;
    let t22252 = -12.0_f64 * t1392 * t6837 + 3.0_f64 * t1392 * t6840 + 3.0_f64 * t1395 * t6832 + 6.0_f64 * t1877 * t5655 + 6.0_f64 * t1879 * t5644 - t22223 * t541 - 24.0_f64 * t22229 * t5652 + 60.0_f64 * t22237 * t5650 - 24.0_f64 * t22240 * t5650 - 12.0_f64 * t22246 * t5650 + 3.0_f64 * t22249 * t539;
    t22252
}
