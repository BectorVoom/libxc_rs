//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 973/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk973<F: Float>(t40641: F, t43072: F, t44855: F, t44857: F, t44860: F, t44861: F, t44862: F, t44863: F, t44864: F, t44865: F, t739: F, t1022: F, t39048: F, t787: F) -> (F, F, F) {
    let t50182 = t44855 - t44857 + F::new(2.0) * t43072 - F::new(2.0) * t40641 + t44860 + t44861 - t44862 + t44863 - t44864 - t44865;
    let t50183 = t739 * t50182;
    let t50194 = t787 * t39048 * t1022;
    (t50182, t50183, t50194)
}
