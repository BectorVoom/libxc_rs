//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1207/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1207(t1468: f64, t17501: f64, t27514: f64, t5919: f64, t17509: f64, t94785: f64, t28589: f64, t4262: f64, t17490: f64, t27520: f64, t27529: f64, t28610: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97713 = t1468 * t17501;
    let t97715 = t27514 * t5919;
    let t97717 = t94785 * t17509;
    let t97719 = t28589 * t4262;
    let t97721 = t27520 * t17490;
    let t97723 = t28610 * t27529;
    (t97713, t97715, t97717, t97719, t97721, t97723)
}
