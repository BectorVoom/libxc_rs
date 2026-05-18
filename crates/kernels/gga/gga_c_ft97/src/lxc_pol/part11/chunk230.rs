//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 230/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk230<F: Float>(t184: F, t648: F, t21: F, t231: F, t240: F, t247: F, t342: F, t343: F, t10: F, t242: F, t351: F, t322: F) -> (F, F, F, F, F, F, F) {
    let t649 = t648 * t184;
    let t650 = t649 * t21;
    let t657 = t231 * t240;
    let t661 = t247 - t342 * t343 * t657 / F::new(4.0);
    let t663 = t10 * t351 * t242;
    let t664 = t663 / F::new(18.0);
    let t665 = F::new(1.0) / t322;
    (t649, t650, t657, t661, t663, t664, t665)
}
