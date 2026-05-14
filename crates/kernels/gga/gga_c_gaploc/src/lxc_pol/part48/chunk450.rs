//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 450/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk450<F: Float>(t1381: F, t997: F, t2876: F, t540: F, t1: F, t2754: F, t106: F, t192: F, t1564: F, t2765: F, t524: F, t188: F, t7930: F, t493: F, t7892: F, t1397: F, t2897: F) -> (F, F, F, F, F, F, F, F) {
    let t8045 = t997 * t1381;
    let t8063 = t2876 * t540;
    let t8070 = t2754 * t1;
    let t8071 = t8070 * t106;
    let t8072 = t8071 * t192;
    let t8097 = t1564 * t2754;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8195 = t493 * t7892;
    let t8229 = t1397 * t2897;
    (t8045, t8063, t8072, t8097, t8155, t8158, t8195, t8229)
}
