//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 494/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk494<F: Float>(t1526: F, t2322: F, t9483: F, t2326: F, t342: F, t630: F, t2427: F, t677: F, t322: F, t668: F, t693: F, t226: F) -> (F, F, F, F, F, F, F) {
    let t9485 = t1526 * t9483 * t2322;
    let t9488 = t342 * t630 * t2326;
    let t9533 = t677 * t2427;
    let t9567 = F::new(1.0) / t322 / t668;
    let t9680 = t693 * t693;
    let t9681 = F::new(1.0) / t9680;
    let t9682 = t226 * t9681;
    (t9485, t9488, t9533, t9567, t9680, t9681, t9682)
}
