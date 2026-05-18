//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 875/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk875<F: Float>(t12738: F, t3459: F, t160: F, t972: F, t1170: F, t1171: F, t1177: F, t168: F) -> (F, F, F, F, F) {
    let t12739 = t12738 * t3459;
    let t12741 = t160 * t972;
    let t12743 = t1170 * t12741 * t1171;
    let t12744 = t12743 * t1177;
    let t12746 = t12741 * t168;
    let t12747 = t1170 * t12746;
    (t12739, t12743, t12744, t12746, t12747)
}
