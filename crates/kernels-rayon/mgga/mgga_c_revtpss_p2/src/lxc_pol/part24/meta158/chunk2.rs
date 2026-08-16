//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 795/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk795(t1082: f64, t6244: f64, t1089: f64, t6271: f64, t1651: f64, t5004: f64, t6258: f64, t378: f64, t6305: f64, t3304: f64, t1668: f64, t1678: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6362 = t1082 * t6244;
    let t6365 = t6271 * t1089;
    let t6368 = t5004 * t1651;
    let t6371 = t1082 * t6258;
    let t6374 = t378 * t6305;
    let t6375 = t6374 * t3304;
    let t6379 = t1678 * t1668 * t1089;
    (t6362, t6365, t6368, t6371, t6374, t6375, t6379)
}
