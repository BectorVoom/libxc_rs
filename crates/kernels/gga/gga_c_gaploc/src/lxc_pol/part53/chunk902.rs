//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 902/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk902<F: Float>(t14517: F, t1960: F, t42506: F, t42509: F, t44202: F, t44207: F, t44221: F, t47096: F, t47097: F, t47105: F, t47112: F, t50930: F, t50931: F, t50933: F, t50934: F, t50983: F, t50984: F, t50985: F, t50986: F, t841: F) -> (F,) {
    let t51072 = 2.0 * t14517 * t1960 * t841 - t42506 - t42509 + t44202 - t44207 - t44221 - 2.0 * t47096 - 2.0 * t47097 + 4.0 * t47105 - 2.0 * t47112 + t50930 + t50931 + t50933 - t50934 - t50983 - t50984 - t50985 - t50986;
    (t51072,)
}
