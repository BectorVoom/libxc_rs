//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1082/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1082(t5: f64, t32578: f64, t9239: f64, t33: f64, t8854: f64, t2240: f64, t7254: f64, t8307: f64, t8513: f64, t31000: f64, t31006: f64, t31013: f64, t31024: f64, t8663: f64, t8856: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32579 = t9239 * t32578;
    let t32582 = t33 * t8854;
    let t32583 = t2240 * t32582;
    let t32587 = t8513 * t8307 * t7254;
    let t32590 = t2240 * t32578;
    let t32594 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t31000 * t8856 - 5.0_f64 / 24.0_f64 * t32579 * t31006 - 5.0_f64 / 36.0_f64 * t32583 * t31013 + 5.0_f64 / 72.0_f64 * t8663 * t32587 + 5.0_f64 / 72.0_f64 * t32590 * t31024);
    (t32579, t32582, t32583, t32587, t32590, t32594)
}
