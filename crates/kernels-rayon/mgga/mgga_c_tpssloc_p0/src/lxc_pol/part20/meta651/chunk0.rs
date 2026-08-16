//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2394/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2394(t13654: f64, t2842: f64, t2844: f64, t912: f64, t10727: f64, t13727: f64, t10731: f64, t13520: f64, t41811: f64, t4359: f64, t41623: f64, t4400: f64) -> (f64, f64, f64, f64, f64) {
    let t49080 = 0.48245938496077605201e2_f64 * t2842 * t13654 * t2844 * t912;
    let t49082 = 6.0_f64 * t13727 * t10727;
    let t49084 = 0.48245938496077605201e2_f64 * t13520 * t10731;
    let t49086 = 6.0_f64 * t41811 * t4359;
    let t49088 = 0.48245938496077605201e2_f64 * t41623 * t4400;
    (t49080, t49082, t49084, t49086, t49088)
}
