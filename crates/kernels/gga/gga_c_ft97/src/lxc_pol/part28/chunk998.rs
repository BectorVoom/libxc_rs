//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 998/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk998<F: Float>(t147647: F, t446: F, t9073: F, t34817: F, t40830: F, t558: F, t34827: F, t376: F, t5890: F, t1369: F, t34835: F, t147122: F, t28: F, t89: F, t34927: F, t148132: F, t32906: F, t7239: F, t7366: F) -> (F, F, F, F, F, F, F) {
    let t148492 = t446 * t9073 * t147647;
    let t148496 = t446 * t40830 * t34817 * t558;
    let t148499 = t5890 * t376 * t34827;
    let t148502 = t1369 * t376 * t34835;
    let t148508 = t89 * t28 * t147122 * t558;
    let t148511 = t89 * t376 * t34927;
    let t148515 = t7366 * t7239 * t32906 * t148132;
    (t148492, t148496, t148499, t148502, t148508, t148511, t148515)
}
