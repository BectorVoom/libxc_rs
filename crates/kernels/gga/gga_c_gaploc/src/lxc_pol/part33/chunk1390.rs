//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1390/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1390<F: Float>(t30808: F, t30809: F, t30812: F, t30820: F, t30823: F, t30827: F, t30833: F, t34603: F, t34607: F, t34609: F, t34612: F, t34614: F, t34621: F, t34623: F, t34626: F, t34628: F) -> F {
    let t38654 = -t30808 + F::cast_from(0.76685851907841499354e0_f64) * t30809 + F::cast_from(0.76685851907841499354e0_f64) * t30812 + F::cast_from(0.38342925953920749677e0_f64) * t30820 - F::cast_from(0.76685851907841499354e0_f64) * t30823 - F::cast_from(0.38342925953920749677e0_f64) * t30827 - t30833 - t34603 - t34607 - t34609 - t34612 - t34614 + t34621 + t34623 + t34626 + t34628;
    t38654
}
