//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1058/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1058<F: Float>(t2101: F, t2142: F, t2133: F, t605: F, t9114: F, t1948: F, t2252: F, t342: F, t511: F, t8639: F, t1526: F, t1944: F, t38308: F, t1970: F, t7705: F, t8779: F) -> (F, F, F, F, F, F, F, F) {
    let t40945 = t2101 * t2142;
    let t41198 = t2101 * t2133;
    let t41269 = t9114 * t605;
    let t41305 = t342 * t2252 * t1948;
    let t41328 = 5.0 / 54.0 * t342 * t8639 * t511;
    let t41332 = t1526 * t38308 * t1944;
    let t41335 = t1526 * t7705 * t1970;
    let t41338 = t1526 * t7705 * t8779;
    (t40945, t41198, t41269, t41305, t41328, t41332, t41335, t41338)
}
