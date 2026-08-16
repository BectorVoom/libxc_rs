//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1963/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963(t1877: f64, t2057: f64, t24344: f64, t2522: f64, t26740: f64, t26756: f64, t28241: f64, t28249: f64, t28972: f64, t4314: f64, t46341: f64, t5397: f64, t7110: f64, t7114: f64, t7475: f64, t7545: f64, t84797: f64, t92276: f64, t98000: f64, t98031: f64, t98046: f64, t98050: f64, t98065: f64, t98082: f64, t98091: f64, t98103: f64) -> f64 {
    let t101283 = 2.0_f64 * t26756 * t98031 + 2.0_f64 * t26756 * t98065 + 3.0_f64 * t4314 * t7110 * t28241 + t1877 * t24344 * t98091 - 3.0_f64 * t84797 * t28249 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t98046 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t98050 + 3.0_f64 * t46341 * t28972 - t1877 * t7114 * t98082 / 2.0_f64 - 3.0_f64 * t26756 * t98000 + t26756 * t98103 + t1877 * t7110 * t5397 / 2.0_f64 - t1877 * t92276 * t7545 + 3.0_f64 * t2522 * t26740 * t7475;
    t101283
}
