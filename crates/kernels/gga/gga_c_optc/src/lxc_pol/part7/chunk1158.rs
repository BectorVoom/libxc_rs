//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1158/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1158<F: Float>(t23: F, t287: F, t8291: F, t8294: F, t8297: F, t2474: F, t2534: F, t845: F, t279: F, t5714: F, t1001: F, t3902: F, t999: F) -> (F, F, F, F) {
    let t24072 = t8291 * t8294 * t23 * t287 * t8297;
    let t24076 = F::cast_from(0.21053604230838734656e2_f64) * t845 * t2474 * t2534;
    let t24088 = F::cast_from(1.0_f64) / t279 / t5714;
    let t24094 = t999 * t3902 * t1001;
    (t24072, t24076, t24088, t24094)
}
