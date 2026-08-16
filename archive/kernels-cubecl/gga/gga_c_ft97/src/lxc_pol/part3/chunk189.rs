//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 189/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk189<F: Float>(t149: F, t165: F, t564: F, t610: F, t614: F, t616: F, t184: F, t169: F, t5: F, t13: F, t171: F) -> (F, F, F, F) {
    let t619 = -t149 * t614 - t165 * t564 - F::cast_from(2.0_f64) * t610 + F::cast_from(2.0_f64) * t616;
    let t620 = t619 * t184;
    let t623 = t5 * t169;
    let t625 = F::cast_from(1.0_f64) / t171 / t13;
    (t619, t620, t623, t625)
}
