//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1037/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1037<F: Float>(t277: F, t33965: F, t11980: F, t11772: F, t29006: F, t11748: F, t19210: F, t2597: F, t11397: F, t761: F, t11979: F, t3074: F, t3775: F, t9538: F, t33258: F, t3698: F, t3780: F) -> (F, F, F, F, F, F, F) {
    let t33966 = t277 * t33965;
    let t33967 = t33966 * t11980;
    let t33969 = t11772 * t29006;
    let t33972 = t11748 * t2597 * t19210;
    let t33975 = t761 * t11397 * t11980;
    let t33977 = t3074 * t11979;
    let t33978 = t33966 * t33977;
    let t33980 = t3775 * t9538;
    let t33983 = t33258 * t3698 * t3780;
    (t33967, t33969, t33972, t33975, t33978, t33980, t33983)
}
