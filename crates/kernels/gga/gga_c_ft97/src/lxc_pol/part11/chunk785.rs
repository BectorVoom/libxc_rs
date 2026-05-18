//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 785/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk785<F: Float>(t10588: F, t10621: F, t845: F, t91: F, t305: F, t631: F, t7242: F, t798: F, t898: F, t2756: F, t856: F, t10246: F) -> (F, F, F, F, F) {
    let t10622 = t10588 + t10621;
    let t10624 = t91 * t845 * t10622;
    let t10631 = F::new(1.0) / t305 / t631 / t898 / t798 / t7242 / F::new(4.0);
    let t10632 = t2756 * t856;
    let t10634 = t91 * t10631 * t10632;
    let t10636 = F::new(2.0) / F::new(9.0) * t10246;
    (t10622, t10624, t10631, t10634, t10636)
}
