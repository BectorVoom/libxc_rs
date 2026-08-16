//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1694/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1694(t5819: f64, t6258: f64, t1469: f64, t22671: f64, t1651: f64, t22688: f64, t1042: f64, t1063: f64, t1066: f64, t11703: f64, t15716: f64, t16081: f64, t16208: f64, t23481: f64, t23911: f64, t247: f64, t3091: f64, t3092: f64, t3116: f64, t3127: f64, t43253: f64, t4801: f64, t4806: f64, t4837: f64, t65581: f64, t65596: f64, t78496: f64, t78910: f64, t78915: f64, t78986: f64, t88091: f64, t88646: f64, t88750: f64) -> (f64, f64, f64, f64) {
    let t88901 = t5819 * t6258;
    let t88916 = t22671 * t1469;
    let t88925 = t22688 * t1651;
    let t88944 = -0.22866142996303859718e-2_f64 * t78910 - 0.11433071498151929859e-2_f64 * t78915 - 0.14291339372689912324e-2_f64 * t3127 * t1042 * t4806 * t88901 + 0.34299214494455789578e-2_f64 * t16081 * t3092 * t78496 * t43253 * t1469 + 0.17149607247227894789e-2_f64 * t78986 - 0.34299214494455789578e-2_f64 * t4837 * t1042 * t4801 * t88750 - 0.11433071498151929859e-2_f64 * t1063 * t1042 * t4801 * t88916 + 0.95275595817932748828e-3_f64 * t1063 * t1042 * t4806 * t88916 - 0.2540682555144873302e-2_f64 * t3127 * t1042 * t16208 * t88925 - 0.57165357490759649296e-2_f64 * t3091 * t11703 * t23481 * t23911 + 0.28582678745379824648e-3_f64 * t65581 + 0.3811023832717309953e-3_f64 * t65596 - 0.85748036236139473944e-3_f64 * t1063 * t247 * t1066 * t88091 - 0.77173232612525526552e-2_f64 * t15716 * t247 * t3116 * t88646;
    (t88901, t88916, t88925, t88944)
}
