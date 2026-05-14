//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1202/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1202<F: Float>(t1466: F, t28954: F, t681: F, t25462: F, t28941: F, t29415: F, t92: F, t10683: F, t10697: F, t13863: F, t15425: F, t25440: F, t2801: F, t2844: F, t28847: F, t28930: F, t28938: F, t28945: F, t44601: F, t6216: F, t6217: F, t6219: F, t6963: F, t7114: F, t875: F, t98357: F, t98380: F, t98389: F) -> (F,) {
    let t112463 = t1466 * t681 * t28954 / 9.0;
    let t112465 = 2.0 / 27.0 * t25462 * t28941;
    let t112479 = t29415 * t92;
    let t112491 = 2.0 / 9.0 * t98357 + t98380 / 54.0 - t112463 - t112465 - t6216 * t28938 * t28945 * t13863 / 3.0 + 48.0 * t44601 * t7114 * t2844 + t6963 * t25440 / 6.0 + t98389 / 27.0 + t6216 * t10683 * t6217 * t15425 - t112479 * t6219 / 9.0 - 12.0 * t10697 * t7114 * t2801 - 24.0 * t10697 * t28847 * t875 - 24.0 * t10697 * t28930 * t875;
    (t112491,)
}
