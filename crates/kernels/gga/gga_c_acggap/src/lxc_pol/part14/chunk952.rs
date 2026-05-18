//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 952/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk952<F: Float>(t30409: F, t30418: F, t31309: F, t525: F, t2016: F, t8622: F, t515: F, t7852: F, t2294: F, t7630: F, t31253: F, t527: F) -> (F, F, F, F, F) {
    let t33857 = t31309 * t30418 * t30409 * t525;
    let t33859 = t2016 * t8622;
    let t33860 = F::new(11.0) / F::new(576.0) * t33859;
    let t33861 = t7852 * t515;
    let t33865 = t7630 * t2294;
    let t33867 = t31253 * t527;
    (t33857, t33860, t33861, t33865, t33867)
}
