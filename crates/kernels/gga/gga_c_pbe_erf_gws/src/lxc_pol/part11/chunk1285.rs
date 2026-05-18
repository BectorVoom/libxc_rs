//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1285/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1285<F: Float>(t49943: F, t49945: F, t49950: F, t49952: F, t49954: F, t49963: F, t49980: F, t49986: F, t50027: F, t50036: F, t50041: F, t50043: F, t50045: F, t50049: F, t50051: F, t50056: F, t50073: F, t50077: F, t50087: F, t50103: F, t50107: F, t50109: F) -> (F, F) {
    let t50582 = -t49943 - t49945 + t49950 + t49952 + t49954 + t49963 + t49980 + t49986 - t50027 + t50036 + t50041;
    let t50583 = -t50043 + t50045 - t50049 + t50051 + t50056 - t50073 + t50077 + t50087 - t50103 - t50107 - t50109;
    (t50582, t50583)
}
