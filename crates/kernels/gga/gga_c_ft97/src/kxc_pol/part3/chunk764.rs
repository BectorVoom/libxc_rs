//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 764/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk764<F: Float>(t79: F, t11136: F, t15632: F, t15636: F, t15640: F, t15643: F, t15649: F, t15652: F, t15658: F, t15665: F, t15669: F, t15682: F, t15710: F, t15712: F, t15716: F, t15720: F, t15724: F, t15727: F, t15777: F, t15782: F, t15786: F, t15789: F, t15793: F, t15882: F, t1594: F, t1603: F, t1624: F, t1631: F, t1669: F, t3019: F, t3076: F, t372: F, t374: F, t401: F, t428: F, t7861: F, t8042: F) -> F {
    let t80 = F::new(0.1e-59) < t79;
    let t15885 = piecewise3::<F>(t80, F::cast_from(0.16027353291807919743e-5_f64) * t15632 * t7861 + F::cast_from(0.23254900946437792e-1_f64) * t1624 * t15636 - F::cast_from(0.13519760450715832853e-3_f64) * t3019 * t15640 + F::cast_from(0.46509801892875584e-2_f64) * t1603 * t1631 * t15643 - F::cast_from(0.11619434043764639964e-3_f64) * t372 * t15649 - F::cast_from(0.23254900946437792e-1_f64) * t1603 * t374 * t15652 + F::cast_from(0.19365723406274399941e-3_f64) * t372 * t15658 + F::cast_from(0.38731446812548799882e-3_f64) * t1603 * t1594 * t15643 - F::cast_from(0.19365723406274399941e-3_f64) * t1624 * t15665 + F::cast_from(0.38731446812548799882e-3_f64) * t372 * t15669 + t15710 + F::new(4.0) * t1669 * t15712 * t401 - F::new(6.0) * t3076 * t15716 * t428 - F::cast_from(0.32253953169881963531e-5_f64) * t372 * t15720 - F::cast_from(0.23254900946437792e-1_f64) * t8042 * t15724 + F::new(2.0) * t3076 * t15727 * t428 - F::cast_from(0.11627450473218896e-1_f64) * t372 * t374 * t15777 + F::cast_from(0.67598802253579164263e-4_f64) * t3019 * t15782 + F::cast_from(0.11627450473218896e-1_f64) * t1624 * t15786 - F::cast_from(0.40559281352147498558e-4_f64) * t15789 * t15682 + F::cast_from(0.13519760450715832853e-3_f64) * t15793 * t11136 + t15882, F::new(0.0));
    t15885
}
