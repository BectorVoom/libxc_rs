//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 918/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk918(t127: f64, t129: f64, t130: f64, t2379: f64, t2496: f64, t2507: f64, t2508: f64, t2514: f64, t2518: f64, t344: f64, t60: f64, t684: f64, t756: f64, t763: f64, t764: f64, t768: f64, t8543: f64, t8547: f64, t8557: f64, t8562: f64, t8566: f64, t8567: f64, t8605: f64, t8611: f64, t8618: f64, t8747: f64) -> f64 {
    let t8750 = -0.39422577999999999998e-2_f64 * t8543 * t2508 - 0.59133866999999999997e-2_f64 * t2507 * t8547 + 0.78845155999999999997e-2_f64 * t127 * t756 * t2514 - 0.59133866999999999997e-2_f64 * t763 * t764 * t2514 - 0.11826773399999999999e-1_f64 * t127 * t129 * t8557 + 0.11826773399999999999e-1_f64 * t344 * t8562 + 0.13140859333333333333e-2_f64 * t8566 * t8567 - 0.19711288999999999999e-2_f64 * t127 * t129 * t8605 - 12.0_f64 * t684 * t2379 + 0.58403819259259259257e-3_f64 * t127 * t8611 * t130 + 0.13140859333333333333e-2_f64 * t127 * t2496 * t768 + 0.21901432222222222225e-3_f64 * t763 * t8618 * t130 - 0.39422577999999999999e-2_f64 * t127 * t756 * t2518 + 0.29566933499999999998e-2_f64 * t763 * t764 * t2518 - 4.0_f64 * t60 * t8747;
    t8750
}
