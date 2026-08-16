//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2538/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2538(t2873: f64, t4587: f64, t11298: f64, t1596: f64, t11466: f64, t1633: f64, t11299: f64, t1609: f64, t51913: f64, t51915: f64, t51973: f64, t52035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52505 = t4587 * t2873;
    let t52508 = t1596 * t11298;
    let t52511 = t11466 * t1633;
    let t52514 = t11299 * t1609;
    let t52546 = 0.69463333333333333334e0_f64 * t51913;
    let t52547 = 0.11577222222222222222e0_f64 * t51915;
    let t52573 = 0.68863333333333333332e0_f64 * t51973;
    let t52597 = 0.13772666666666666666e1_f64 * t52035;
    (t52505, t52508, t52511, t52514, t52546, t52547, t52573, t52597)
}
