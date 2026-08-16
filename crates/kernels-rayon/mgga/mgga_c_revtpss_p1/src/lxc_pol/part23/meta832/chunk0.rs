//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2693/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2693(t1647: f64, t16558: f64, t1078: f64, t6258: f64, t3057: f64, t6343: f64, t3046: f64, t20112: f64, t342: f64, t15669: f64, t1678: f64, t1679: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67972 = t1647 * t16558;
    let t68018 = t1078 * t6258;
    let t68022 = t3057 * t6343;
    let t68072 = t3046 * t6343;
    let t68138 = t342 * t20112;
    let t68144 = t15669 * t1678;
    let t68170 = t994 * t1679;
    (t67972, t68018, t68022, t68072, t68138, t68144, t68170)
}
