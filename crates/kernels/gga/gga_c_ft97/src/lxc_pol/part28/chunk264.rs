//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 264/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk264<F: Float>(t160: F, t358: F, t171: F, t360: F, t70: F, t170: F, t180: F, t11: F, t625: F) -> (F, F, F, F, F) {
    let t2222 = t160 * t358;
    let t2247 = F::new(1.0) / t171 / t360;
    let t2248 = t2247 * t70;
    let t2251 = F::new(5.0) / F::new(18.0) * t170 * t2248 * t180;
    let t2252 = t11 * t625;
    (t2222, t2247, t2248, t2251, t2252)
}
