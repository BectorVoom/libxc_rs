//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 958/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk958<F: Float>(t1526: F, t20518: F, t7705: F, t11262: F, t20507: F, t4656: F, t20522: F, t342: F, t630: F, t20989: F, t5: F, t21669: F, t8392: F) -> (F, F, F, F, F, F) {
    let t78653 = t1526 * t7705 * t20518;
    let t78678 = t1526 * t11262 * t20507;
    let t78681 = t1526 * t7705 * t4656;
    let t78700 = t342 * t630 * t20522;
    let t78929 = t5 * t20989;
    let t79007 = t8392 * t21669;
    (t78653, t78678, t78681, t78700, t78929, t79007)
}
