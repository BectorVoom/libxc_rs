//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1004/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1004<F: Float>(t1017: F, t139418: F, t28: F, t89: F, t26768: F, t5778: F, t32967: F, t3408: F, t34939: F, t376: F, t34931: F, t147730: F, t39749: F, t446: F, t7339: F, t1369: F, t2112: F) -> (F, F, F, F, F, F, F, F) {
    let t148593 = t89 * t28 * t139418 * t1017;
    let t148597 = t89 * t28 * t5778 * t26768;
    let t148601 = t89 * t28 * t32967 * t3408;
    let t148604 = t89 * t376 * t34939;
    let t148607 = t89 * t376 * t34931;
    let t148611 = t446 * t39749 * t147730;
    let t148613 = t7339 * t3408;
    let t148616 = t1369 * t28 * t2112 * t148613;
    (t148593, t148597, t148601, t148604, t148607, t148611, t148613, t148616)
}
