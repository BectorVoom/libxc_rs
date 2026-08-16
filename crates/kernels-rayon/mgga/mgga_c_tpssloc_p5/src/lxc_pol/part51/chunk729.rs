//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 729/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk729(t1873: f64, t2314: f64, t5113: f64, t1268: f64, t6534: f64, t6515: f64, t6517: f64, t671: f64, t1271: f64, t191: f64, t192: f64) -> (f64, f64, f64) {
    let t6867 = 2.0_f64 * t2314 * t1873;
    let t6869 = 2.0_f64 * t5113 * t1873;
    let t6871 = 2.0_f64 * t1268 * t6534;
    let t6872 = 2.0_f64 * t6517 * t671 + t6515 + t6867 + t6869 + t6871;
    let t6875 = t1271 * t191;
    let t6876 = t6875 * t192;
    (t6872, t6875, t6876)
}
