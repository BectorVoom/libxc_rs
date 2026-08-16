//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2267/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267(t26179: f64, t6535: f64, t2314: f64, t25994: f64, t12823: f64, t7461: f64, t25980: f64, t4034: f64, t12813: f64, t89: f64, t1874: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91739 = 4.0_f64 * t26179 * t6535;
    let t91747 = 4.0_f64 * t2314 * t25994;
    let t91749 = 2.0_f64 * t12823 * t7461;
    let t91752 = 4.0_f64 * t4034 * t25980;
    let t91753 = t89 * t12813;
    let t91755 = 2.0_f64 * t91753 * t1874;
    let t91757 = 4.0_f64 * t26179 * t6525;
    (t91739, t91747, t91749, t91752, t91755, t91757)
}
