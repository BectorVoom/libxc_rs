//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1061/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1061<F: Float>(t1664: F, t66: F, t3018: F, t62: F, t1594: F, t401: F, t929: F, t422: F, t8715: F, t1595: F, t37487: F, t11120: F, t1655: F, t7857: F, t1808: F, t965: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45566 = t1664 * t66;
    let t45572 = t3018 * t62;
    let t45573 = t1594 * t45572;
    let t45574 = t929 * t401;
    let t45751 = t8715 * t422;
    let t45886 = t37487 * t1595;
    let t45887 = t45886 * t11120;
    let t45890 = t7857 * t1655;
    let t45891 = t45890 * t11120;
    let t46177 = t965 * t1808;
    (t45566, t45573, t45574, t45751, t45886, t45887, t45890, t45891, t46177)
}
