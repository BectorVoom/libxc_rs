//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1065/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1065<F: Float>(t20837: F, t3491: F, t91: F, t446: F, t569: F, t85456: F, t2205: F, t85465: F, t1969: F, t86906: F, t85474: F, t1985: F, t27: F, t86681: F, t89: F) -> (F, F, F, F, F, F) {
    let t86986 = t91 * t3491 * t20837;
    let t86989 = t446 * t569 * t85456;
    let t86992 = t446 * t2205 * t85465;
    let t86995 = t446 * t1969 * t86906;
    let t86998 = t446 * t569 * t85474;
    let t87002 = t89 * t27 * t1985 * t86681;
    (t86986, t86989, t86992, t86995, t86998, t87002)
}
