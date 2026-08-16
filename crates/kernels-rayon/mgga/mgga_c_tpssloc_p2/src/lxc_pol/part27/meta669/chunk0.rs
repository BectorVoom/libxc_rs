//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2363/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2363(t1983: f64, t2019: f64, t55169: f64, t510: f64, t652: f64, t86604: f64, t26114: f64, t6535: f64, t26179: f64, t2314: f64, t25994: f64, t12823: f64, t7461: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91730 = t1983 * t2019 * t55169;
    let t91735 = 2.0_f64 * t652 * t510 * t86604;
    let t91737 = 4.0_f64 * t26114 * t6535;
    let t91739 = 4.0_f64 * t26179 * t6535;
    let t91747 = 4.0_f64 * t2314 * t25994;
    let t91749 = 2.0_f64 * t12823 * t7461;
    (t91730, t91735, t91737, t91739, t91747, t91749)
}
