//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 469/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk469<F: Float>(t1349: F, t1362: F, t149: F, t7309: F, t7315: F, t7342: F, t7346: F, t7396: F, t7401: F, t7408: F, t7412: F, t7414: F, t184: F, t2252: F, t342: F, t344: F) -> (F, F, F) {
    let t7419 = t7309 * t1362 / 6.0 - t1349 * t7315 / 3.0 + t1349 * t7342 / 6.0 + t1349 * t7346 / 3.0 - t149 * t7412 + 2.0 * t7414 - 4.0 * t7396 + 4.0 * t7401 - 2.0 * t7408;
    let t7420 = t7419 * t184;
    let t7704 = t342 * t2252 * t344 / 18.0;
    (t7419, t7420, t7704)
}
