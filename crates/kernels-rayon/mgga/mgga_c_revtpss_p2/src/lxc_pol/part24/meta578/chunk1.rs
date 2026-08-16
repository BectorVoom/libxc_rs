//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1782/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782(t90347: f64, t90506: f64, t90600: f64, t90868: f64, t23842: f64, t24792: f64, t24610: f64, t1715: f64, t1774: f64, t6622: f64, t1042: f64, t1247: f64, t1250: f64, t12866: f64, t17235: f64, t17351: f64, t17353: f64, t17693: f64, t17799: f64, t20795: f64, t24773: f64, t3604: f64, t3611: f64, t3626: f64, t3711: f64, t44458: f64, t44510: f64, t482: f64, t5274: f64, t5340: f64, t5819: f64, t69839: f64, t69910: f64, t69964: f64, t82932: f64, t90001: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90870 = t90347 + t90506 + t90600 + t90868;
    let t90881 = t23842 * t24792;
    let t90885 = t24610 * t24792;
    let t90889 = t1715 * t1774;
    let t90894 = t1715 * t6622;
    let t90900 = -0.34299214494455789578e-2_f64 * t5340 * t3626 * t20795 * t44458 * t5819 - 0.57165357490759649296e-3_f64 * t69910 + 0.85748036236139473944e-3_f64 * t5274 * t24773 + 0.21437009059034868486e-3_f64 * t1247 * t1042 * t482 * t90870 * t1250 + 0.2540682555144873302e-2_f64 * t3711 * t1042 * t17235 * t90001 + 0.22866142996303859718e-2_f64 * t82932 - 0.34299214494455789578e-2_f64 * t17693 * t17799 * t90881 + 0.34299214494455789577e-2_f64 * t12866 * t17799 * t90885 + 0.34299214494455789578e-2_f64 * t44510 * t69839 * t3604 * t90889 + 0.17149607247227894789e-2_f64 * t17351 * t17353 * t3611 * t90894 + 0.28582678745379824648e-3_f64 * t69964;
    (t90870, t90881, t90885, t90889, t90894, t90900)
}
