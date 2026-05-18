//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 478/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk478<F: Float>(t158: F, t2754: F, t2796: F, t501: F, t1381: F, t997: F, t2876: F, t540: F, t1: F, t106: F, t192: F, t1564: F) -> (F, F, F, F, F, F) {
    let t8025 = t158 * t2754;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    let t8063 = t2876 * t540;
    let t8070 = t2754 * t1;
    let t8071 = t8070 * t106;
    let t8072 = t8071 * t192;
    let t8097 = t1564 * t2754;
    (t8025, t8042, t8045, t8063, t8072, t8097)
}
