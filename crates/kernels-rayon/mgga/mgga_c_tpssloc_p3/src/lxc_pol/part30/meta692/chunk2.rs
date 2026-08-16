//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2206/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2206(t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25013: f64, t2522: f64, t25372: f64, t28249: f64, t28448: f64, t28459: f64, t5397: f64, t606: f64, t6666: f64, t6670: f64, t6671: f64, t81483: f64, t98046: f64, t98050: f64, t98054: f64, t98059: f64, t98065: f64, t98071: f64, t98075: f64, t98079: f64, t98082: f64, t98086: f64) -> f64 {
    let t98090 = -3.0_f64 * t81483 * t28249 + t1877 * t28448 * t606 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t98046 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t98050 - t1877 * t98054 * t6671 / 2.0_f64 - 6.0_f64 * t25013 * t98059 - t1877 * t23290 * t28459 + 2.0_f64 * t25372 * t98065 - t98071 + t1877 * t6666 * t5397 / 2.0_f64 - t1877 * t6670 * t98075 / 2.0_f64 - 3.0_f64 * t22959 * t98079 - t1877 * t6670 * t98082 / 2.0_f64 - t1877 * t6670 * t98086 / 2.0_f64;
    t98090
}
