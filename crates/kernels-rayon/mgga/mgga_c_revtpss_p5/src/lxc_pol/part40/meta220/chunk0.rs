//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 874/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk874(t3117: f64, t4911: f64, t1012: f64, t1014: f64, t4579: f64, t3252: f64, t4574: f64, t140: f64, t1655: f64, t1011: f64, t1656: f64, t3115: f64, t3234: f64, t3241: f64, t3245: f64, t4887: f64, t4892: f64, t4896: f64, t4899: f64, t4902: f64, t4907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4912 = t3117 * t4911;
    let t4915 = t1012 * t1014;
    let t4916 = t4915 * t4579;
    let t4919 = t1012 * t3252;
    let t4920 = t4919 * t4574;
    let t4924 = t140 * t1655;
    let t4925 = t1011 * t4924;
    let t4928 = -t3241 * t1656 / 108.0_f64 + t1011 * t4887 / 288.0_f64 + 0.42874018118069736972e-3_f64 * t4892 * t4896 - 0.21437009059034868486e-3_f64 * t4899 * t4902 - 0.21437009059034868486e-3_f64 * t3115 * t4907 - 0.21437009059034868486e-3_f64 * t3115 * t4912 - t1011 * t4916 / 144.0_f64 + t1011 * t4920 / 216.0_f64 - 0.76220476654346199061e-3_f64 * t3234 + t4925 / 864.0_f64 + t3245 / 864.0_f64;
    (t4912, t4915, t4916, t4919, t4920, t4924, t4928)
}
