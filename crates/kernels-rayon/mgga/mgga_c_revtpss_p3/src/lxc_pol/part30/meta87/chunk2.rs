//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 548/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk548(t1811: f64, t225: f64, t494: f64, t1280: f64, t1774: f64, t1287: f64, t1794: f64, t487: f64, t489: f64, t1234: f64, t1285: f64, t1770: f64, t460: f64, t490: f64) -> (f64, f64, f64, f64, f64) {
    let t1812 = t1811 * t225;
    let t1813 = t1812 * t494;
    let t1818 = t1280 * t1774;
    let t1822 = t487 * t1794 * t1287;
    let t1825 = t489 * t1811;
    let t1828 = 0.65854491829355115987e0_f64 * t1770 * t490 - 0.65854491829355115987e0_f64 * t1234 * t1818 + 0.65854491829355115987e0_f64 * t1285 * t1822 + 0.65854491829355115987e0_f64 * t460 * t1825;
    (t1813, t1818, t1822, t1825, t1828)
}
