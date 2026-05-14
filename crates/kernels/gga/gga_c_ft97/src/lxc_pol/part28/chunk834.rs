//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 834/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk834<F: Float>(t136516: F, t78: F, t32300: F, t409: F, t173: F, t22557: F, t32273: F, t7195: F, t32250: F, t92335: F, t1613: F, t92354: F, t1624: F, t2258: F, t36363: F, t36390: F, t5567: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t136517 = t136516 * t78;
    let t136520 = t32300 * t409;
    let t136531 = t22557 * t7195 * t173 * t32273;
    let t136555 = t92335 * t32250;
    let t136558 = t1613 * sigma0;
    let t136559 = t92354 * t136558;
    let t136560 = t1624 * t136559;
    let t136561 = t36363 * t2258;
    let t136565 = t36390 * t5567;
    (t136517, t136520, t136531, t136555, t136559, t136560, t136561, t136565)
}
