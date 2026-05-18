//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 949/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk949<F: Float>(t5373: F, t663: F, t2660: F, t5346: F, t16563: F, t7062: F, t7069: F, t5038: F, t5211: F, t617: F, t7483: F, t4892: F, t610: F, t7514: F) -> (F, F, F, F, F) {
    let t17608 = F::new(8.0) / F::new(15.0) * t5373 * t663;
    let t17609 = t2660 * t5346;
    let t17610 = F::new(32.0) / F::new(15.0) * t17609;
    let t17613 = F::new(16.0) / F::new(9.0) * t7062 * t7069 * t16563;
    let t17617 = F::new(64.0) / F::new(15.0) * t5211 * t7483 * t617 * t5038;
    let t17621 = F::new(32.0) / F::new(15.0) * t7062 * t7514 * t610 * t4892;
    (t17608, t17610, t17613, t17617, t17621)
}
