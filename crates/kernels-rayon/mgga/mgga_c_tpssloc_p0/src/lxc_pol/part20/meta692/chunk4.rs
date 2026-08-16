//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2639/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2639(t28: f64, t265: f64, t504: f64, t47655: f64, t51129: f64, t51803: f64, t51825: f64, t51826: f64, t51836: f64, t51867: f64, t51885: f64, t53735: f64, t10150: f64, t1081: f64, t11122: f64, t11957: f64, t1260: f64, t12606: f64, t13493: f64, t1409: f64, t1534: f64, t15844: f64, t1649: f64, t1768: f64, t2250: f64, t3231: f64, t3644: f64, t3966: f64, t4324: f64, t45872: f64, t47668: f64, t47670: f64, t47672: f64, t47674: f64, t47676: f64, t506: f64, t5099: f64, t52: f64, t607: f64, t9258: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t53739 = piecewise3(t505, t51129 + t51803 + t51825 + t51826 + t51836 + t51867 + t51885 + t53735, t47655);
    let t53757 = piecewise3(t401, t47655 * t28 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t13493 * t1081 + 3.0_f64 / 2.0_f64 * t4324 * t3231 + t1534 * t11122 / 2.0_f64 + t10150 * t1649 / 2.0_f64 - t47668 - t47670 + t47672 + t47674 - t47676, t53739 * t52 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t15844 * t607 - 3.0_f64 / 2.0_f64 * t5099 * t2250 - t1768 * t9258 / 2.0_f64 - t11957 * t1409 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t3644 * t3966 - 3.0_f64 / 2.0_f64 * t1260 * t12606 - t506 * t45872 / 2.0_f64);
    t53757
}
