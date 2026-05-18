//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 990/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk990<F: Float>(t1369: F, t32955: F, t376: F, t137087: F, t7366: F, t7370: F, t1389: F, t5842: F, t2178: F, t7390: F, t24073: F, t7309: F) -> (F, F, F, F, F, F) {
    let t139526 = t1369 * t376 * t32955;
    let t139533 = t7366 * t137087 * t7370;
    let t139534 = F::new(10.0) / F::new(27.0) * t139533;
    let t139563 = t5842 * t1389;
    let t139573 = t7390 * t2178;
    let t139600 = t7309 * t24073;
    (t139526, t139533, t139534, t139563, t139573, t139600)
}
