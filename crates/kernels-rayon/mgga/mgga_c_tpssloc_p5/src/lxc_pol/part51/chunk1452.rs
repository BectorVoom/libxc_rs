//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1452/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1452(t31776: f64, t96797: f64, t1983: f64, t33136: f64, t7217: f64, t33623: f64, t6876: f64, t33214: f64, t7057: f64, t25985: f64, t8607: f64, t120071: f64, t122088: f64, t122094: f64, t122583: f64, t2096: f64, t22461: f64, t24980: f64, t26103: f64, t26969: f64, t27226: f64, t6517: f64, t7042: f64, t7802: f64, t8450: f64) -> f64 {
    let t122587 = 2.0_f64 * t96797 * t31776;
    let t122589 = t1983 * t7217 * t33136;
    let t122590 = t6876 * t33623;
    let t122593 = 2.0_f64 * t33214 * t7057;
    let t122595 = 3.0_f64 * t8607 * t25985;
    let t122596 = t120071 * t2096 - 2.0_f64 * t22461 * t7802 - 2.0_f64 * t24980 * t7042 - 2.0_f64 * t26103 * t7802 + 3.0_f64 * t26969 * t8450 - 2.0_f64 * t27226 * t6517 + t122088 + t122094 + t122583 + t122587 - t122589 - t122590 - t122593 + t122595;
    t122596
}
