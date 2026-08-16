//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1283/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1283(t22893: f64, t23164: f64, t31377: f64, t31390: f64, t6547: f64, t23030: f64, t31381: f64, t23110: f64, t23185: f64, t31385: f64, t22690: f64, t23171: f64, t31376: f64) -> (f64, f64, f64, f64, f64) {
    let t114666 = t23164 * t22893 * t31377;
    let t114670 = t6547 * t31390;
    let t114672 = t23030 * t31381;
    let t114673 = 0.26044789391763585244e-1_f64 * t114672;
    let t114680 = t23185 * t23110 * t31385;
    let t114688 = t23171 * t22690 * t31376;
    (t114666, t114670, t114673, t114680, t114688)
}
