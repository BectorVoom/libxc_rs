//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1197/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1197(t12832: f64, t27625: f64, t7978: f64, t27641: f64, t4425: f64, t94588: f64, t12844: f64, t27583: f64, t27585: f64, t94904: f64, t7968: f64, t95006: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95045 = t7978 * t12832 * t27625;
    let t95052 = t7978 * t4425 * t27641;
    let t95088 = 0.51588271604938271604e-3_f64 * t94588;
    let t95115 = t27583 * t12844 * t27585;
    let t95123 = t7978 * t94904;
    let t95125 = t7968 * t94904;
    let t95127 = t7978 * t95006;
    (t95045, t95052, t95088, t95115, t95123, t95125, t95127)
}
