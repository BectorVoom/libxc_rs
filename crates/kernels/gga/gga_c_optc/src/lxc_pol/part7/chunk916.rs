//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 916/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk916<F: Float>(t8627: F, t8678: F, t1056: F, t1037: F, t8552: F, t8557: F, t8560: F, t8564: F, t8567: F, t8571: F, t8574: F, t8576: F, t8579: F, t8585: F) -> (F, F, F, F) {
    let t8679 = t8627 + t8678;
    let t8680 = t8679 * t1056;
    let t8682 = F::new(1.0) * t1037 * t8680;
    let t8683 = t8552 - t8557 - t8560 + t8564 - t8567 + t8571 + t8574 + t8576 + t8579 - t8585 + t8682;
    (t8679, t8680, t8682, t8683)
}
