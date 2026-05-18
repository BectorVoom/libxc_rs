//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 915/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk915<F: Float>(t30727: F, t7670: F, t7676: F, t7724: F, t2056: F, t7600: F, t2074: F, t30456: F, t2035: F, t420: F, t7544: F, t1095: F, t30572: F, t30573: F, t7458: F) -> (F, F, F, F, F, F, F) {
    let t31470 = t30727 * t7670;
    let t31471 = F::new(0.38586616306262763276e-2) * t31470;
    let t31472 = t7676 * t7724;
    let t31473 = F::new(0.19293308153131381638e-2) * t31472;
    let t31477 = t7600 * t2056;
    let t31479 = t30456 * t2074;
    let t31491 = t2035 * t420;
    let t31494 = t7676 * t7544;
    let t31495 = F::new(0.28303283060643736861e-2) * t31494;
    let t31498 = t30572 * t7458 * t1095 * t30573;
    (t31471, t31473, t31477, t31479, t31491, t31495, t31498)
}
