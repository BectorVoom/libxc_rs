//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1782/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1782<F: Float>(t90347: F, t90506: F, t90600: F, t90868: F, t23842: F, t24792: F, t24610: F, t1715: F, t1774: F, t6622: F, t1042: F, t1247: F, t1250: F, t12866: F, t17235: F, t17351: F, t17353: F, t17693: F, t17799: F, t20795: F, t24773: F, t3604: F, t3611: F, t3626: F, t3711: F, t44458: F, t44510: F, t482: F, t5274: F, t5340: F, t5819: F, t69839: F, t69910: F, t69964: F, t82932: F, t90001: F) -> (F, F, F, F, F, F) {
    let t90870 = t90347 + t90506 + t90600 + t90868;
    let t90881 = t23842 * t24792;
    let t90885 = t24610 * t24792;
    let t90889 = t1715 * t1774;
    let t90894 = t1715 * t6622;
    let t90900 = -F::cast_from(0.34299214494455789578e-2_f64) * t5340 * t3626 * t20795 * t44458 * t5819 - F::cast_from(0.57165357490759649296e-3_f64) * t69910 + F::cast_from(0.85748036236139473944e-3_f64) * t5274 * t24773 + F::cast_from(0.21437009059034868486e-3_f64) * t1247 * t1042 * t482 * t90870 * t1250 + F::cast_from(0.2540682555144873302e-2_f64) * t3711 * t1042 * t17235 * t90001 + F::cast_from(0.22866142996303859718e-2_f64) * t82932 - F::cast_from(0.34299214494455789578e-2_f64) * t17693 * t17799 * t90881 + F::cast_from(0.34299214494455789577e-2_f64) * t12866 * t17799 * t90885 + F::cast_from(0.34299214494455789578e-2_f64) * t44510 * t69839 * t3604 * t90889 + F::cast_from(0.17149607247227894789e-2_f64) * t17351 * t17353 * t3611 * t90894 + F::cast_from(0.28582678745379824648e-3_f64) * t69964;
    (t90870, t90881, t90885, t90889, t90894, t90900)
}
