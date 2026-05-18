//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1187/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1187<F: Float>(t40225: F, t38674: F, t544: F, t9287: F, t2365: F, t38272: F, t7025: F, t41938: F, t41941: F, t41942: F, t41945: F, t41948: F, t41950: F, t41952: F, t41954: F, t41958: F) -> F {
    let t47963 = F::new(0.15337170381568299871e1) * t40225;
    let t47964 = t544 * t38674;
    let t47965 = t47964 * t9287;
    let t47968 = t7025 * t2365 * t38272;
    let t47972 = F::new(0.47667319935800568892e0) * t41938 + t41941 + F::new(0.35750489951850426669e0) * t41942 + t41945 - t47963 + F::new(0.14896037479937677779e-1) * t47965 + F::new(0.14896037479937677779e-1) * t47968 - t41948 - t41950 - t41952 + F::new(0.25561950635947166451e0) * t41954 - F::new(0.44688112439813033337e-1) * t41958;
    t47972
}
