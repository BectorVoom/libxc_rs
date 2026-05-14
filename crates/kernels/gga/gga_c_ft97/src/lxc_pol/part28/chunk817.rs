//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 817/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk817<F: Float>(t2001: F, t94400: F, t5818: F, t23831: F, t3392: F, t23700: F, t172: F, t549: F, t72: F, t128: F, t1691: F, t14: F, t2057: F, t2178: F, t5929: F, t40280: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t94401 = t2001 * t94400;
    let t94429 = t5818 * t94400;
    let t94514 = t23831 * t94400;
    let t94524 = t3392 * t94400;
    let t94530 = t23831 * t23700;
    let t94535 = t2001 * t23700;
    let t94552 = t549 * t172 * t72;
    let t94760 = t128 * t1691;
    let t94765 = t2057 * t14;
    let t95021 = t5929 * t2178;
    let t95262 = t91 * t40280;
    (t94401, t94429, t94514, t94524, t94530, t94535, t94552, t94760, t94765, t95021, t95262)
}
