//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1003/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1003(t12365: f64, t374: f64, t1039: f64, t3570: f64, t1149: f64, t2449: f64, t11554: f64, t986: f64, t11496: f64, t3574: f64, t983: f64, t2892: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12366 = t12365 * t374;
    let t12367 = t1039 * t3570;
    let t12368 = t2449 * t1149;
    let t12383 = t11554 * t986;
    let t12391 = t11496 * t986;
    let t12395 = t3574 * t983;
    let t12414 = t797 * t2892;
    (t12366, t12367, t12368, t12383, t12391, t12395, t12414)
}
