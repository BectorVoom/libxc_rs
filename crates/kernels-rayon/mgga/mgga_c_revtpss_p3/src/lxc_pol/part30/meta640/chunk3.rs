//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2227/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2227(t104646: f64, t17727: f64, t17423: f64, t29097: f64, t17789: f64, t29100: f64, t17416: f64, t7624: f64, t17214: f64, t17484: f64, t17580: f64, t17760: f64, t29037: f64, t29040: f64, t29083: f64, t3620: f64, t3640: f64, t3644: f64, t97149: f64, t97261: f64) -> (f64, f64) {
    let t104647 = t17727 * t104646;
    let t104651 = 0.11433071498151929859e-2_f64 * t29097 * t17423;
    let t104653 = 0.57165357490759649296e-3_f64 * t29100 * t17789;
    let t104658 = t7624 * t17416;
    let t104666 = -0.85748036236139473944e-3_f64 * t97149 * t17580 + 0.42874018118069736972e-3_f64 * t97261 * t17484 - 0.95275595817932748826e-3_f64 * t104647 * t17760 + t104651 - t104653 + 0.15244095330869239812e-2_f64 * t29083 * t3640 + 0.30488190661738479624e-2_f64 * t29083 * t3644 + 0.6351706387862183255e-4_f64 * t104658 + 0.47637797908966374413e-3_f64 * t29037 * t3620 - 0.2540682555144873302e-2_f64 * t29083 * t3620 - 0.57165357490759649296e-3_f64 * t29040 * t17214;
    (t104647, t104666)
}
