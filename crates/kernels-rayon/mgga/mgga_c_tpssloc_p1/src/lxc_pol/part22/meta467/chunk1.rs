//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1853/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1853(t1336: f64, t1814: f64, t1838: f64, t1840: f64, t19815: f64, t20595: f64, t20616: f64, t20622: f64, t20625: f64, t20630: f64, t20632: f64, t20635: f64, t20638: f64, t20643: f64, t20645: f64, t20648: f64, t20651: f64, t5234: f64, t5334: f64, t5344: f64, t544: f64, t564: f64, t6378: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64, t6458: f64) -> f64 {
    let t20661 = -6.0_f64 * t1336 * t20622 + 6.0_f64 * t1336 * t20625 - t1336 * t20630 - 3.0_f64 * t1336 * t20635 - t1336 * t20643 - 3.0_f64 * t1336 * t20645 - 3.0_f64 * t1336 * t20648 + 6.0_f64 * t1336 * t20651 + 3.0_f64 * t1814 * t6458 - 3.0_f64 * t1838 * t19815 + 3.0_f64 * t1840 * t6378 + t20595 * t564 + t20616 * t544 - 3.0_f64 * t20632 * t5344 + 6.0_f64 * t20638 * t5334 + 6.0_f64 * t5234 * t6448 - 6.0_f64 * t5234 * t6451 - 3.0_f64 * t5234 * t6454 - 3.0_f64 * t5234 * t6456;
    t20661
}
