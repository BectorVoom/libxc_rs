//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1297/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1297(t22574: f64, t26162: f64, t55183: f64, t6535: f64, t9348: f64, t12734: f64, t12823: f64, t107: f64, t835: f64, t240: f64, t656: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81426 = 18.0_f64 * t22574 * t26162 * t55183;
    let t81430 = 6.0_f64 * t9348 * t6535;
    let t81432 = 12.0_f64 * t12734 * t6535;
    let t81434 = 6.0_f64 * t12823 * t6535;
    let t81437 = t835 * t107;
    let t81438 = 154.0_f64 / 27.0_f64 * t81437;
    let t81439 = t240 * t656;
    let t81440 = t81439 * t666;
    (t81426, t81430, t81432, t81434, t81438, t81440)
}
