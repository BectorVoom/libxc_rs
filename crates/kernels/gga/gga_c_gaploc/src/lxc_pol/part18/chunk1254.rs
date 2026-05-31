//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1254/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1254<F: Float>(t23575: F, t2972: F, t10805: F, t5552: F, t1960: F, t2728: F, t3073: F, t7822: F, t7332: F, t8862: F, t11125: F, t11127: F, t1955: F, t31478: F, t31480: F, t31483: F, t31485: F, t32090: F, t32091: F, t32093: F, t32095: F, t32099: F, t3511: F, t5549: F, t841: F) -> F {
    let t32723 = F::cast_from(4.0_f64) * t23575 * t2972;
    let t32731 = F::cast_from(4.0_f64) * t5552 * t10805;
    let t32734 = F::cast_from(4.0_f64) * t1960 * t3073 * t2728;
    let t32736 = F::cast_from(2.0_f64) * t7822 * t3073;
    let t32740 = F::cast_from(2.0_f64) * t8862 * t7332;
    let t32741 = F::cast_from(4.0_f64) * t11125 * t1960 * t841 - F::cast_from(2.0_f64) * t11125 * t1955 + F::cast_from(4.0_f64) * t11127 * t5552 - t3511 * t5549 - t31478 - t31480 - t31483 + t31485 - t32090 + t32091 + t32093 - t32095 - t32099 + t32723 + t32731 + t32734 - t32736 + t32740;
    t32741
}
