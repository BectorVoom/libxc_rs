//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 996/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk996<F: Float>(t1466: F, t1982: F, t1490: F, t303: F, t1498: F, t1983: F, t2002: F, t27475: F, t5732: F, t7914: F, t6176: F, t3961: F, t6140: F, t1014: F, t8171: F, t1380: F, t1650: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28524 = t1982 * t1466;
    let t28525 = t28524 * t1490;
    let t28526 = t303 * t28525;
    let t28528 = t1983 * t1498;
    let t28529 = t303 * t28528;
    let t28531 = t27475 * t2002;
    let t28532 = t303 * t28531;
    let t28534 = t7914 * t5732;
    let t28535 = t6176 * t28534;
    let t28544 = t3961 * t6140;
    let t28547 = t1014 * t8171;
    let t28549 = t1650 * t1380;
    (t28524, t28525, t28526, t28528, t28529, t28531, t28532, t28534, t28535, t28544, t28547, t28549)
}
