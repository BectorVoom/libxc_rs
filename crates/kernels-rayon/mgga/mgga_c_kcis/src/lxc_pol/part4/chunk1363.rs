//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1363/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1363(t17520: f64, t571: f64, t3393: f64, t5989: f64, t12417: f64, t1517: f64, t1650: f64, t531: f64, t5867: f64, t833: f64, t17244: f64, t509: f64) -> (f64, f64, f64, f64, f64) {
    let t17521 = t571 * t17520;
    let t17540 = t3393 * t5989;
    let t17543 = t1517 * t12417 * t1650;
    let t17546 = t5867 * t531;
    let t17548 = t1517 * t17546 * t833;
    let t17552 = t509 * t17244;
    (t17521, t17540, t17543, t17548, t17552)
}
