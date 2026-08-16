//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 978/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk978(t3941: f64, t5493: f64, t8326: f64, t1458: f64, t1851: f64, t576: f64, t33191: f64, t2022: f64, t5456: f64, t8657: f64, t33185: f64, t33656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127627 = 27.0_f64 * t3941 * t8326 * t5493;
    let t127630 = t1851 * t1458;
    let t127643 = t576 * t5493;
    let t127646 = 27.0_f64 * t33191;
    let t127647 = t2022 * t5456;
    let t127669 = 27.0_f64 * t127643 * t8657;
    let t127671 = 54.0_f64 * t33185 * t33656;
    (t127627, t127630, t127646, t127647, t127669, t127671)
}
