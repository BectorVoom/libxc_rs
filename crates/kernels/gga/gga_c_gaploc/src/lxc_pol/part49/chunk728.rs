//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 728/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk728<F: Float>(t1415: F, t8247: F, t7892: F, t9439: F, t9448: F, t4348: F, t997: F, t1033: F, t5558: F, t1381: F, t2796: F, t3209: F, t701: F, t830: F, t2530: F, t935: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26984 = t1415 * t8247;
    let t27003 = t9439 * t7892;
    let t27007 = t9448 * t7892;
    let t27214 = t997 * t4348;
    let t27229 = t1033 * t5558;
    let t27232 = t2796 * t1381;
    let t27997 = t3209 * t701;
    let t28002 = t830 * t3209;
    let t28013 = t935 * t2530;
    (t26984, t27003, t27007, t27214, t27229, t27232, t27997, t28002, t28013)
}
