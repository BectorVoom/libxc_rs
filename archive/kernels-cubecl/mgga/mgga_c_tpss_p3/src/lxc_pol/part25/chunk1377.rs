//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1377/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1377<F: Float>(t13133: F, t1338: F, t13473: F, t13554: F, t1600: F, t1800: F, t18547: F, t20319: F, t20357: F, t20371: F, t20374: F, t20396: F, t2056: F, t20640: F, t20642: F, t21011: F, t21180: F, t21236: F, t21750: F, t21880: F, t24128: F, t3493: F, t3499: F, t4541: F, t5801: F, t5809: F, t5816: F, t6243: F, t626: F, t6318: F, t6409: F, t645: F, t68958: F, t69069: F, t69383: F) -> F {
    let t72682 = -F::cast_from(4.0_f64) * t21180 * t5816 - F::cast_from(4.0_f64) * t13133 * t6318 - F::cast_from(4.0_f64) * t13554 * t6318 - F::cast_from(4.0_f64) * t3493 * t20396 - F::cast_from(4.0_f64) * t3493 * t20374 - F::cast_from(2.0_f64) * t69069 * t1800 - F::cast_from(2.0_f64) * t69383 * t1800 - F::cast_from(2.0_f64) * t21236 * t5809 - F::cast_from(2.0_f64) * t6243 * t20642 - F::cast_from(6.0_f64) * t18547 * t24128 * t21011 + F::cast_from(6.0_f64) * t18547 * t20357 * t68958 - F::cast_from(2.0_f64) * t626 * t21750 * t645 - F::cast_from(4.0_f64) * t626 * t1600 * t20319 - F::cast_from(4.0_f64) * t2056 * t21880 - F::cast_from(4.0_f64) * t3499 * t21880 - F::cast_from(4.0_f64) * t626 * t20640 * t1338 - F::cast_from(4.0_f64) * t5801 * t13473 - F::cast_from(4.0_f64) * t3493 * t20371 + F::cast_from(2.0_f64) * t6409 * t4541;
    t72682
}
