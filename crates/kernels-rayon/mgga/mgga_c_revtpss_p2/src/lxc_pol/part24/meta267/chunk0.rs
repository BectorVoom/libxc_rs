//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1039/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1039(t3566: f64, t488: f64, t1276: f64, t1774: f64, t1209: f64, t1828: f64, t3736: f64, t1811: f64, t17306: f64, t487: f64, t116: f64, t5876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    let t17986 = t1209 * t488;
    let t17987 = t3736 * t1828;
    let t17995 = t3566 * t1811;
    let t18059 = t17306 * t487;
    let t18245 = t5876 * t116;
    (t17973, t17974, t17986, t17987, t17995, t18059, t18245)
}
