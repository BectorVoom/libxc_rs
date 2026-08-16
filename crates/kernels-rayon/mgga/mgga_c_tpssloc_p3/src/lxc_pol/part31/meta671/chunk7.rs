//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2008/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2008(t1307: f64, t2094: f64, t671: f64, t7786: f64, t100990: f64, t1266: f64, t1459: f64, t19289: f64, t1983: f64, t20127: f64, t2036: f64, t2040: f64, t22574: f64, t24432: f64, t24987: f64, t24990: f64, t26905: f64, t26969: f64, t27188: f64, t28826: f64, t28959: f64, t29252: f64, t4026: f64, t4037: f64, t510: f64, t5361: f64, t5450: f64, t6287: f64, t652: f64, t6876: f64, t7040: f64, t7042: f64, t7156: f64, t75214: f64, t7685: f64, t7890: f64, t7900: f64, t7943: f64, t84733: f64, t96356: f64, t97789: f64) -> (f64, f64) {
    let t102336 = t1307 * t2094;
    let t102344 = t7786 * t671;
    let t102366 = -6.0_f64 * t22574 * t24432 * t97789 - t7040 * t6287 - t2036 * t19289 + 6.0_f64 * t1983 * t84733 * t28826 + 6.0_f64 * t1983 * t26905 * t24990 + 6.0_f64 * t6876 * t29252 + 6.0_f64 * t1983 * t102336 * t28826 - t5450 * t7156 - 2.0_f64 * t652 * t510 * t100990 - 4.0_f64 * t102344 * t1459 - 4.0_f64 * t27188 * t4037 - 2.0_f64 * t7042 * t20127 - 2.0_f64 * t28959 * t1266 - 2.0_f64 * t24987 * t7943 - 3.0_f64 * t22574 * t24432 * t75214 + 6.0_f64 * t7685 * t26969 - 2.0_f64 * t4026 * t7890 + 2.0_f64 * t7900 * t5361 - 4.0_f64 * t96356 * t2040;
    (t102344, t102366)
}
