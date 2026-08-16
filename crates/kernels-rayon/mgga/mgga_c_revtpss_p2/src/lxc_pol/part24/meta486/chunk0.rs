//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1479/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479(t20849: f64, t3754: f64, t3781: f64, t6564: f64, t3766: f64, t17191: f64, t5219: f64, t3566: f64, t6695: f64, t487: f64, t69636: f64, t17306: f64, t1811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t72270 = t20849 * t3754;
    let t72326 = t6564 * t3781;
    let t72370 = t6564 * t3766;
    let t72386 = t5219 * t17191;
    let t72767 = t3566 * t6695;
    let t72802 = t69636 * t487;
    let t72874 = t17306 * t1811;
    (t72270, t72326, t72370, t72386, t72767, t72802, t72874)
}
