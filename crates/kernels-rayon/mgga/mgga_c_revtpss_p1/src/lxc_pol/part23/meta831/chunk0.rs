//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2691/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691(t1063: f64, t11986: f64, t247: f64, t6096: f64, t20112: f64, t359: f64, t19572: f64, t3302: f64, t12046: f64, t1678: f64, t342: f64, t1086: f64, t6343: f64, t994: f64) -> (f64, f64, f64, f64, f64) {
    let t67575 = t1063 * t247 * t11986 * t6096;
    let t67595 = t359 * t20112;
    let t67599 = t19572 * t3302;
    let t67644 = t342 * t12046 * t1678;
    let t67652 = t994 * t1086 * t6343;
    (t67575, t67595, t67599, t67644, t67652)
}
