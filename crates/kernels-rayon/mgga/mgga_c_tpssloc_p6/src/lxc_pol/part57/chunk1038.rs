//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1038/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1038(t22685: f64, t31618: f64, t6330: f64, t6637: f64, t122448: f64, t1825: f64, t22633: f64, t6976: f64, t115435: f64, t122475: f64, t122503: f64, t127382: f64, t127386: f64, t127391: f64, t1336: f64, t31636: f64, t33289: f64, t5234: f64, t6378: f64, t6420: f64, t8634: f64) -> f64 {
    let t128860 = t22685 * t6637 * t31618 * t6330;
    let t128865 = t22633 * t6976 * t122448 * t1825;
    let t128874 = t115435 + t6378 * t8634 + 0.49348022005446793095e-1_f64 * t128860 + t127382 - 0.38381794893125283518e-1_f64 * t122503 + 0.3289868133696452873e-1_f64 * t128865 - t127386 - t127391 - t1336 * t31636 * t6420 - 2.0_f64 * t5234 * t33289 - 2.0_f64 * t1336 * t122475 * t1825;
    t128874
}
