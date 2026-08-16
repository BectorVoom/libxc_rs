//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1033/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1033<F: Float>(t12276: F, t12325: F, t224: F, t3751: F, t856: F, t1531: F, t2876: F, t2097: F, t3039: F, t123: F, t3689: F, t3720: F) -> (F, F, F, F, F, F, F) {
    let t12326 = t12276 + t12325;
    let t12327 = t224 * t12326;
    let t12339 = t856 * t3751;
    let t12881 = t2876 * t1531;
    let t13045 = t3039 * t2097;
    let t13777 = t3689 * t123;
    let t13846 = t3720 * t123;
    (t12326, t12327, t12339, t12881, t13045, t13777, t13846)
}
