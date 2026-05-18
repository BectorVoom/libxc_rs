//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 214/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk214<F: Float>(t663: F, t231: F, t294: F, t301: F, t342: F, t343: F, t10: F, t296: F, t351: F, t295: F, t668: F) -> (F, F, F, F, F, F, F, F) {
    let t764 = t663 / F::new(3.0);
    let t784 = t231 * t294;
    let t788 = t301 - t342 * t343 * t784 / F::new(4.0);
    let t790 = t10 * t351 * t296;
    let t791 = t790 / F::new(18.0);
    let t792 = t295 * t668;
    let t797 = t294 * t294;
    let t798 = F::new(1.0) / t797;
    (t764, t784, t788, t790, t791, t792, t797, t798)
}
