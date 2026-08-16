//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1078/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1078(t32206: f64, t33926: f64, t1903: f64, t32211: f64, t5673: f64, t1892: f64, t8477: f64, t8590: f64, t552: f64, t125: f64, t246: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33927 = t32206 * t33926;
    let t33930 = t5673 * t32211 * t1903;
    let t33931 = t32206 * t33930;
    let t33943 = t8477 * t1892;
    let t33959 = t33943 * t8590;
    let t33960 = t33959 * t552;
    let t33962 = t125 * t1903;
    let t33963 = t246 * t33962;
    let t33964 = t551 * t33963;
    (t33927, t33930, t33931, t33943, t33959, t33960, t33962, t33963, t33964)
}
