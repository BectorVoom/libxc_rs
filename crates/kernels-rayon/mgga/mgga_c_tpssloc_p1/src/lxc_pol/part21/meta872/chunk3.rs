//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3215/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3215(t28: f64, t265: f64, t504: f64, t59618: f64, t64473: f64, t64510: f64, t64534: f64, t64545: f64, t66885: f64, t66886: f64, t66891: f64, t66901: f64, t1081: f64, t1260: f64, t12606: f64, t13493: f64, t1409: f64, t15844: f64, t1649: f64, t16558: f64, t17133: f64, t1768: f64, t18196: f64, t19276: f64, t2250: f64, t2756: f64, t3231: f64, t3644: f64, t3966: f64, t47676: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t55677: f64, t5669: f64, t59627: f64, t59629: f64, t59631: f64, t5966: f64, t607: f64, t6279: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t66905 = piecewise3(t505, t64473 + t64510 + t64534 + t64545 + t66885 + t66886 + t66891 + t66901, t59618);
    let t66921 = piecewise3(t401, t59618 * t28 / 2.0_f64 + t17133 * t1081 + t5669 * t3231 / 2.0_f64 + t13493 * t1649 - t59627 - t59629 + t59631 + t2756 * t5966 / 2.0_f64 + t873 * t18196 - t47676, t66905 * t52 / 2.0_f64 - t19276 * t607 - t6279 * t2250 / 2.0_f64 - t15844 * t1409 - 2.0_f64 * t5099 * t3966 - t1768 * t12606 - t3644 * t5398 / 2.0_f64 - t1260 * t16558 - t506 * t55677 / 2.0_f64);
    t66921
}
