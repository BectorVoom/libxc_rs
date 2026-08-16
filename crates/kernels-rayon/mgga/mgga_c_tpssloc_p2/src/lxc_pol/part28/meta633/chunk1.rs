//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2002/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002(t1877: f64, t2057: f64, t24191: f64, t24339: f64, t2522: f64, t25905: f64, t25921: f64, t25930: f64, t25934: f64, t26740: f64, t26756: f64, t6841: f64, t7110: f64, t7114: f64, t84797: f64, t89850: f64, t89888: f64, t89892: f64, t89911: f64, t89917: f64, t89978: f64, t92356: f64, t92359: f64, t92362: f64, t92364: f64) -> f64 {
    let t93211 = 2.0_f64 * t26756 * t89850 - t1877 * t7114 * t89978 / 2.0_f64 - t92356 - t1877 * t24339 * t25934 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t89888 + 6.0_f64 * t24191 * t89917 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t89911 + 3.0_f64 * t2522 * t2057 * t89892 + 3.0_f64 * t2522 * t7110 * t25905 - t1877 * t24339 * t25930 + t92359 - 3.0_f64 * t84797 * t25921 + 3.0_f64 * t2522 * t26740 * t6841 - t92362 + t92364;
    t93211
}
