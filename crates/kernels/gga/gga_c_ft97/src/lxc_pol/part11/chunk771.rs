//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 771/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk771<F: Float>(t1589: F, t1636: F, t89: F, t375: F, t8184: F, t7752: F, t23: F, t32075: F, t1588: F, t27: F, t7837: F, t7999: F, t1608: F, t1613: F, t373: F, t408: F) -> (F, F, F, F, F, F, F, F) {
    let t37421 = t89 * t1636 * t1589;
    let t37422 = 8.0 / 9.0 * t37421;
    let t37424 = t89 * t375 * t8184;
    let t37427 = t89 * t375 * t7752;
    let t37429 = t23 * t32075;
    let t37430 = t1588 * t1588;
    let t37433 = t89 * t27 * t37429 * t37430;
    let t37435 = t7837 * t7999;
    let t37443 = t1608 * t408 * t1613 * t373;
    (t37421, t37422, t37424, t37427, t37430, t37433, t37435, t37443)
}
