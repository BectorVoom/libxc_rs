//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 360/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk360<F: Float>(t1033: F, t747: F, t1052: F, t841: F, t1: F, t2931: F, t787: F, t2954: F, t549: F, t1048: F, t1628: F, t1029: F, t1589: F) -> (F, F, F, F, F, F, F) {
    let t2969 = t1033 * t747;
    let t2972 = t1052 * t841;
    let t2975 = t2931 * t1;
    let t2976 = t787 * t2975;
    let t2979 = t549 * t2954;
    let t2989 = t1628 * t1048;
    let t2992 = t1589 * t1029;
    (t2969, t2972, t2975, t2976, t2979, t2989, t2992)
}
