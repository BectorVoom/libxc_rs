//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 983/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk983<F: Float>(t2138: F, t322: F, t8004: F, t8107: F, t1264: F, t2147: F, t2229: F, t15407: F, t7942: F, t9427: F, t31935: F, t7963: F) -> (F, F, F, F) {
    let t33128 = t2138 * t8004 * t8107 * t322;
    let t33132 = t2138 * t2147 * t2229 * t1264;
    let t33138 = t7942 * t9427 * t15407;
    let t33144 = t7963 * t9427 * t31935;
    (t33128, t33132, t33138, t33144)
}
