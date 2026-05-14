//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 883/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk883<F: Float>(t1018: F, t86: F, t9526: F, t1024: F, t3038: F, t978: F, t3368: F, t2861: F, t3195: F, t3230: F, t3234: F, t3318: F, t1093: F, t341: F, t3206: F, t9429: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9562 = t86 * t9526 * t1018;
    let t9563 = t9562 * t1024;
    let t9565 = t3038 * t978;
    let t9568 = t3368 * sigma0;
    let t9572 = t2861 * t3195;
    let t9574 = t2861 * t3230;
    let t9576 = t2861 * t3234;
    let t9581 = t2861 * t3318;
    let t9586 = t1093 * t1093;
    let t9587 = 1.0 / t9586;
    let t9588 = t341 * t9587;
    let t9589 = t9588 * sigma0;
    let t9600 = t9429 * t3206;
    (t9562, t9563, t9565, t9568, t9572, t9574, t9576, t9581, t9587, t9588, t9589, t9600)
}
