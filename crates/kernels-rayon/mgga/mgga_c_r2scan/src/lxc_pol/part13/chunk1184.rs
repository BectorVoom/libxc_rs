//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1184/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1184(t40217: f64, t10899: f64, t11770: f64, t2201: f64, t2834: f64, t3316: f64, t10820: f64, t26088: f64, t20407: f64, t2161: f64, t2841: f64, t625: f64) -> (f64, f64, f64, f64, f64) {
    let t40218 = 0.10975748638225852664e-1_f64 * t40217;
    let t40220 = t2201 * t10899 * t11770;
    let t40222 = t2834 * t3316;
    let t40223 = 0.23115257973478049502e0_f64 * t40222;
    let t40224 = t26088 * t10820;
    let t40228 = t2161 * t20407 * t2841 * t625;
    (t40218, t40220, t40223, t40224, t40228)
}
