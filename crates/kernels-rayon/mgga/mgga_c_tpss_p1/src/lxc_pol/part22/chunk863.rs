//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 863/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk863(t1270: f64, t1625: f64, t1630: f64, t5716: f64, t1642: f64, t5721: f64, t1646: f64, t5728: f64, t1649: f64, t1705: f64, t935: f64, t1791: f64, t6090: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6245 = t1270 * t1625;
    let t6249 = t5716 * t1630;
    let t6251 = t5721 * t1642;
    let t6253 = t5728 * t1646;
    let t6259 = t1705 * t1649;
    let t6260 = t6259 * t935;
    let t6304 = t1791 * t6090;
    (t6245, t6249, t6251, t6253, t6259, t6260, t6304)
}
