//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 423/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk423<F: Float>(t240: F, t668: F, t505: F, t231: F, t713: F, t1526: F, t2319: F, t2320: F, t342: F, t343: F, t719: F, t718: F) -> (F, F, F, F, F) {
    let t2321 = t240 * t668;
    let t2322 = t2321 * t505;
    let t2326 = t231 * t713;
    let t2330 = t719 - t2319 - t1526 * t2320 * t2322 / F::new(12.0) - t342 * t343 * t2326 / F::new(4.0);
    let t2331 = t2330 * t718;
    (t2321, t2322, t2326, t2330, t2331)
}
