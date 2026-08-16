//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 728/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk728<F: Float>(t33319: F, t684: F, t9770: F, t6118: F, t33302: F, t9942: F, t1434: F, t193: F, t2506: F, t33307: F, t747: F, t7484: F) -> (F, F, F, F, F, F, F) {
    let t33321 = t9770 * t33319 * t684;
    let t33322 = t6118 * t33321;
    let t33324 = t9942 * t33302;
    let t33326 = t1434 * t193 * t33324;
    let t33328 = t2506 * t33307;
    let t33330 = t1434 * t193 * t33328;
    let t33332 = t7484 * t747;
    (t33321, t33322, t33324, t33326, t33328, t33330, t33332)
}
