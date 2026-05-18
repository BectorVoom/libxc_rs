//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1148/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1148<F: Float>(t47243: F, t6066: F, t6111: F, t10914: F, t10915: F, t41448: F, t44134: F, t44138: F, t44142: F, t44144: F, t44145: F, t44148: F, t44149: F, t44150: F, t44151: F) -> F {
    let t47549 = t6111 * t6066 * t47243;
    let t47552 = t10914 * t10915 * t47243;
    let t47555 = F::new(0.31952438294933958064e0) * t41448;
    let t47556 = t44134 + F::new(0.42900587942220512003e1) * t47549 - F::new(0.21450293971110256001e1) * t47552 + t44138 + t44142 + t44144 - F::new(0.10725146985555128001e1) * t44145 + t47555 - t44148 + t44149 + t44150 + t44151;
    t47556
}
