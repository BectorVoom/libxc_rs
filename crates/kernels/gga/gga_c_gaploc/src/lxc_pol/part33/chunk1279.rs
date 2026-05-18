//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1279/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1279<F: Float>(t1966: F, t33760: F, t590: F, t28878: F, t28880: F, t2714: F, t8634: F, t2718: F, t24817: F, t955: F, t14626: F, t2087: F, t3503: F) -> (F, F, F, F, F, F, F) {
    let t33763 = F::new(0.51123901271894332902e1) * t1966 * t33760 * t590;
    let t33773 = F::new(0.12780975317973583226e0) * t28878;
    let t33774 = F::new(0.63904876589867916128e-1) * t28880;
    let t33786 = F::new(0.71500979903700853338e0) * t2714 * t8634;
    let t33788 = F::new(0.71500979903700853338e0) * t2718 * t8634;
    let t33790 = F::new(0.35750489951850426669e0) * t955 * t24817;
    let t33799 = F::new(0.30674340763136599741e1) * t2087 * t14626 * t3503;
    (t33763, t33773, t33774, t33786, t33788, t33790, t33799)
}
