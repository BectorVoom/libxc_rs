//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 859/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk859<F: Float>(t1407: F, t16562: F, t7062: F, t7063: F, t5920: F, t5953: F, t1: F, t1478: F, t119: F, t671: F, t1999: F, t762: F) -> (F, F, F, F, F, F) {
    let t16563 = t16562 * t1407;
    let t16566 = F::new(32.0) / F::new(15.0) * t7062 * t7063 * t16563;
    let t16567 = t5953 * t5920;
    let t16569 = t1478 * t1;
    let t16572 = F::new(0.28503734567901234566e-4) * t16569 * t119 * t671;
    let t16574 = F::new(0.44134814814814814813e-2) * t762 * t1999;
    (t16563, t16566, t16567, t16569, t16572, t16574)
}
