//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 241/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk241<F: Float>(t668: F, t761: F, t342: F, t630: F, t784: F, t294: F, t505: F, t231: F, t824: F, t1526: F, t2320: F, t343: F, t830: F) -> (F, F, F, F, F, F) {
    let t2607 = t761 * t668;
    let t2638 = t342 * t630 * t784 / F::cast_from(12.0_f64);
    let t2639 = t294 * t668;
    let t2640 = t2639 * t505;
    let t2644 = t231 * t824;
    let t2648 = t830 - t2638 - t1526 * t2320 * t2640 / F::cast_from(12.0_f64) - t342 * t343 * t2644 / F::cast_from(4.0_f64);
    (t2607, t2638, t2639, t2640, t2644, t2648)
}
