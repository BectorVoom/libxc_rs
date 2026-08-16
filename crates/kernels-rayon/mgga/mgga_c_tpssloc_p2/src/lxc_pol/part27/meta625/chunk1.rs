//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2108/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2108(t16524: f64, t23896: f64, t12813: f64, t1458: f64, t7010: f64, t84004: f64, t86582: f64, t86606: f64, t86610: f64, t86612: f64, t86614: f64, t86616: f64, t86619: f64, t86622: f64, t86625: f64, t86629: f64, t86631: f64, t86633: f64, t86635: f64, t86637: f64) -> f64 {
    let t86639 = 27.0_f64 * t16524 * t23896;
    let t86640 = t86582 + t86606 + 0.135e2_f64 * t84004 * t1458 + t86610 + t86612 + t86614 + t86616 + t86619 + t86622 + t86625 + 0.135e2_f64 * t7010 * t12813 + t86629 + t86631 + t86633 + t86635 + t86637 + t86639;
    t86640
}
