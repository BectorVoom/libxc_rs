//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 995/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk995<F: Float>(t2386: F, t3689: F, t544: F, t6514: F, t4130: F, t2482: F, t9272: F, t12063: F, t1424: F, t2299: F, t10525: F, t10526: F, t47803: F) -> (F, F, F, F) {
    let t47846 = t544 * t6514 * t3689 * t2386;
    let t47848 = t4130 * t3689;
    let t47850 = t9272 * t47848 * t2482;
    let t47854 = t544 * t2299 * t12063 * t1424;
    let t47860 = t10525 * t10526 * t47803;
    (t47846, t47850, t47854, t47860)
}
