//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2284/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2284(t25994: f64, t7458: f64, t28817: f64, t6876: f64, t1983: f64, t28826: f64, t83859: f64, t26149: f64, t7685: f64, t16524: f64, t26545: f64, t1873: f64, t66958: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100840 = 4.0_f64 * t7458 * t25994;
    let t100854 = 6.0_f64 * t6876 * t28817;
    let t100861 = 6.0_f64 * t1983 * t83859 * t28826;
    let t100863 = 2.0_f64 * t7685 * t26149;
    let t100871 = 54.0_f64 * t16524 * t26545;
    let t100873 = 0.135e2_f64 * t66958 * t1873;
    (t100840, t100854, t100861, t100863, t100871, t100873)
}
