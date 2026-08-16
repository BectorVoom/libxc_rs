//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 764/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk764(t79: f64, t11136: f64, t15632: f64, t15636: f64, t15640: f64, t15643: f64, t15649: f64, t15652: f64, t15658: f64, t15665: f64, t15669: f64, t15682: f64, t15710: f64, t15712: f64, t15716: f64, t15720: f64, t15724: f64, t15727: f64, t15777: f64, t15782: f64, t15786: f64, t15789: f64, t15793: f64, t15882: f64, t1594: f64, t1603: f64, t1624: f64, t1631: f64, t1669: f64, t3019: f64, t3076: f64, t372: f64, t374: f64, t401: f64, t428: f64, t7861: f64, t8042: f64) -> f64 {
    let t80 = 0.1e-59_f64 < t79;
    let t15885 = piecewise3(t80, 0.16027353291807919743e-5_f64 * t15632 * t7861 + 0.23254900946437792e-1_f64 * t1624 * t15636 - 0.13519760450715832853e-3_f64 * t3019 * t15640 + 0.46509801892875584e-2_f64 * t1603 * t1631 * t15643 - 0.11619434043764639964e-3_f64 * t372 * t15649 - 0.23254900946437792e-1_f64 * t1603 * t374 * t15652 + 0.19365723406274399941e-3_f64 * t372 * t15658 + 0.38731446812548799882e-3_f64 * t1603 * t1594 * t15643 - 0.19365723406274399941e-3_f64 * t1624 * t15665 + 0.38731446812548799882e-3_f64 * t372 * t15669 + t15710 + 4.0_f64 * t1669 * t15712 * t401 - 6.0_f64 * t3076 * t15716 * t428 - 0.32253953169881963531e-5_f64 * t372 * t15720 - 0.23254900946437792e-1_f64 * t8042 * t15724 + 2.0_f64 * t3076 * t15727 * t428 - 0.11627450473218896e-1_f64 * t372 * t374 * t15777 + 0.67598802253579164263e-4_f64 * t3019 * t15782 + 0.11627450473218896e-1_f64 * t1624 * t15786 - 0.40559281352147498558e-4_f64 * t15789 * t15682 + 0.13519760450715832853e-3_f64 * t15793 * t11136 + t15882, 0.0_f64);
    t15885
}
