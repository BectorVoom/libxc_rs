//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1384/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1384<F: Float>(t10445: F, t10456: F, t13119: F, t13220: F, t13228: F, t1760: F, t1830: F, t18690: F, t18697: F, t18707: F, t18714: F, t19000: F, t19620: F, t20227: F, t20343: F, t20371: F, t20379: F, t2056: F, t20640: F, t3493: F, t3499: F, t4341: F, t4525: F, t544: F, t5706: F, t5801: F, t5815: F, t5936: F, t6243: F, t626: F, t6328: F, t645: F, t66051: F, t67557: F, t67586: F, t7798: F) -> F {
    let t67589 = F::cast_from(6.0_f64) * t5706 * t20227 - F::cast_from(2.0_f64) * t3493 * t18697 - F::cast_from(4.0_f64) * t2056 * t20379 - F::cast_from(4.0_f64) * t3499 * t20379 - F::cast_from(4.0_f64) * t626 * t4341 * t5815 - F::cast_from(12.0_f64) * t19620 * t18690 * t66051 - t10445 * t1830 - t1760 * t19000 * t4525 - F::cast_from(2.0_f64) * t1760 * t5936 * t13119 - F::cast_from(2.0_f64) * t626 * t1830 * t13220 - F::cast_from(4.0_f64) * t2056 * t20371 - F::cast_from(4.0_f64) * t626 * t20640 * t645 - F::cast_from(4.0_f64) * t3493 * t18707 - F::cast_from(2.0_f64) * t7798 * t6328 - F::cast_from(4.0_f64) * t10456 * t6328 - F::cast_from(4.0_f64) * t2056 * t20343 - F::cast_from(2.0_f64) * t5801 * t13228 + F::cast_from(2.0_f64) * t6243 * t18714 + (t67557 + t67586) * t544;
    t67589
}
