//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1458/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1458(t13126: f64, t487: f64, t460: f64, t3754: f64, t5219: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64, t1209: f64, t1811: f64, t1269: f64, t1770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17948 = t13126 * t487;
    let t17949 = t460 * t17948;
    let t17958 = t5219 * t3754;
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    let t17986 = t1209 * t488;
    let t17995 = t3566 * t1811;
    let t18005 = t1770 * t1269;
    (t17949, t17958, t17973, t17974, t17986, t17995, t18005)
}
