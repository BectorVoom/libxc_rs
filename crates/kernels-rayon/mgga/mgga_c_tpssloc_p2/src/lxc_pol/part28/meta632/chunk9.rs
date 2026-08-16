//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1998/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1998(t13191: f64, t13196: f64, t1484: f64, t1530: f64, t1877: f64, t2057: f64, t2379: f64, t24335: f64, t24344: f64, t2522: f64, t25374: f64, t2553: f64, t26744: f64, t2745: f64, t2749: f64, t4314: f64, t57893: f64, t58009: f64, t58071: f64, t7114: f64, t7845: f64, t84766: f64, t84791: f64, t84800: f64, t86713: f64, t86717: f64, t868: f64, t86815: f64, t92276: f64, t93000: f64) -> f64 {
    let t93099 = 12.0_f64 * t13191 * t2057 * t4314 + 6.0_f64 * t13196 * t2057 * t4314 + 3.0_f64 * t1484 * t24335 * t2522 - t1530 * t1877 * t84791 + 4.0_f64 * t1877 * t24344 * t58009 + 2.0_f64 * t1877 * t24344 * t86713 + 4.0_f64 * t1877 * t25374 * t84800 - t1877 * t26744 * t2745 + 2.0_f64 * t1877 * t2749 * t93000 - 6.0_f64 * t1877 * t84766 * t86717 - 2.0_f64 * t1877 * t868 * t92276 + 6.0_f64 * t2379 * t4314 * t7845 + 3.0_f64 * t2522 * t2553 * t7845 - 6.0_f64 * t2522 * t57893 * t7114 - 6.0_f64 * t2522 * t58071 * t7114 - 3.0_f64 * t2522 * t7114 * t86815;
    t93099
}
