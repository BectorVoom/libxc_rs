//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1390/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1390(t30808: f64, t30809: f64, t30812: f64, t30820: f64, t30823: f64, t30827: f64, t30833: f64, t34603: f64, t34607: f64, t34609: f64, t34612: f64, t34614: f64, t34621: f64, t34623: f64, t34626: f64, t34628: f64) -> f64 {
    let t38654 = -t30808 + 0.76685851907841499354e0_f64 * t30809 + 0.76685851907841499354e0_f64 * t30812 + 0.38342925953920749677e0_f64 * t30820 - 0.76685851907841499354e0_f64 * t30823 - 0.38342925953920749677e0_f64 * t30827 - t30833 - t34603 - t34607 - t34609 - t34612 - t34614 + t34621 + t34623 + t34626 + t34628;
    t38654
}
