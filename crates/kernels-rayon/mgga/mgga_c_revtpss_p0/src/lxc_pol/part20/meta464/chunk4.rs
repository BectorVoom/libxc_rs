//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1768/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1768(t675: f64, t9898: f64, t268: f64, t4101: f64, t543: f64, t14192: f64, t555: f64, t786: f64, t9994: f64, t10023: f64, t4003: f64, t10115: f64, t1441: f64) -> (f64, f64, f64, f64) {
    let t47366 = t675 * t9898;
    let t47369 = t4101 * t268 * t47366 * t543;
    let t47371 = t14192 * t555;
    let t47372 = t786 * t47371;
    let t47375 = t47372 * t268 * t47366 * t9994;
    let t47379 = t10023 * t268 * t47366 * t4003;
    let t47381 = t10115 * t1441;
    (t47369, t47375, t47379, t47381)
}
