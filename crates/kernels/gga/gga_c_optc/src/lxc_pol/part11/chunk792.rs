//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 792/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk792<F: Float>(t1256: F, t12979: F, t9477: F, t13062: F, t1271: F, t4623: F) -> (F, F, F, F) {
    let t16315 = t12979 * t1256;
    let t16318 = 0.35089340384731224426e1 * t9477;
    let t16319 = 0.17544670192365612213e1 * t13062;
    let t16323 = t4623 * t1271;
    (t16315, t16318, t16319, t16323)
}
