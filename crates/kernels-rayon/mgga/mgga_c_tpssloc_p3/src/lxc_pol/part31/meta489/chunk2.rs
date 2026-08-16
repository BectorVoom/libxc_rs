//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1671/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1671(t1393: f64, t1459: f64, t1849: f64, t1983: f64, t2040: f64, t2079: f64, t22574: f64, t26114: f64, t26898: f64, t26902: f64, t26906: f64, t26967: f64, t26969: f64, t26974: f64, t26977: f64, t4037: f64, t510: f64, t5361: f64, t650: f64, t6876: f64, t7042: f64, t7166: f64, t7218: f64, t7685: f64, t7890: f64, t7900: f64, t7941: f64) -> f64 {
    let t26982 = t1393 * t7900 - 2.0_f64 * t1459 * t26977 + t1849 * t7166 + 3.0_f64 * t1983 * t26898 - t1983 * t26902 + 3.0_f64 * t1983 * t26906 + 3.0_f64 * t1983 * t26969 - 2.0_f64 * t2040 * t26114 + t2079 * t5361 - 3.0_f64 * t22574 * t26974 - t26967 * t510 - 2.0_f64 * t4037 * t7042 - t650 * t7890 + t6876 * t7941 + t7218 * t7685;
    t26982
}
