//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1384/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1384(t10445: f64, t10456: f64, t13119: f64, t13220: f64, t13228: f64, t1760: f64, t1830: f64, t18690: f64, t18697: f64, t18707: f64, t18714: f64, t19000: f64, t19620: f64, t20227: f64, t20343: f64, t20371: f64, t20379: f64, t2056: f64, t20640: f64, t3493: f64, t3499: f64, t4341: f64, t4525: f64, t544: f64, t5706: f64, t5801: f64, t5815: f64, t5936: f64, t6243: f64, t626: f64, t6328: f64, t645: f64, t66051: f64, t67557: f64, t67586: f64, t7798: f64) -> f64 {
    let t67589 = 6.0_f64 * t5706 * t20227 - 2.0_f64 * t3493 * t18697 - 4.0_f64 * t2056 * t20379 - 4.0_f64 * t3499 * t20379 - 4.0_f64 * t626 * t4341 * t5815 - 12.0_f64 * t19620 * t18690 * t66051 - t10445 * t1830 - t1760 * t19000 * t4525 - 2.0_f64 * t1760 * t5936 * t13119 - 2.0_f64 * t626 * t1830 * t13220 - 4.0_f64 * t2056 * t20371 - 4.0_f64 * t626 * t20640 * t645 - 4.0_f64 * t3493 * t18707 - 2.0_f64 * t7798 * t6328 - 4.0_f64 * t10456 * t6328 - 4.0_f64 * t2056 * t20343 - 2.0_f64 * t5801 * t13228 + 2.0_f64 * t6243 * t18714 + (t67557 + t67586) * t544;
    t67589
}
