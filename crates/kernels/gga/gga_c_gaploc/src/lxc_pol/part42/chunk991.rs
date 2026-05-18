//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 991/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk991<F: Float>(t14440: F, t747: F, t12035: F, t8045: F, t12270: F, t12277: F, t1960: F, t2969: F, t2972: F, t3073: F, t331: F, t3749: F, t38892: F, t45974: F, t45976: F, t45983: F, t45988: F, t45990: F, t45992: F, t45993: F, t45997: F, t49980: F, t49983: F, t50338: F, t50356: F, t50373: F, t50407: F, t50421: F, t50435: F, t50454: F, t50465: F, t841: F, t8440: F) -> (F, F) {
    let t50470 = t14440 * t747;
    let t50475 = F::new(4.0) * t8045 * t12035;
    let t50478 = -F::new(2.0) * t8440 * t3749 - t49980 + F::new(4.0) * t1960 * t3073 * t3749 + t49983 + t45974 + t45976 - F::new(2.0) * t2969 * t12270 + (t50338 + t50356 + t50373 + t50407 + t50421 + t50435 + t50454 + t50465) * t331 + t45983 - t50470 * t841 - F::new(2.0) * t12277 * t3073 + t45988 + t45990 - t45992 - t45993 - t45997 - t50475 + F::new(4.0) * t38892 * t2972;
    (t50475, t50478)
}
