//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 847/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk847<F: Float>(t25491: F, t7581: F, t1466: F, t24986: F, t2336: F, t34290: F, t2252: F, t342: F, t7574: F, t34305: F, t630: F, t1774: F, t6343: F, t7570: F, t1526: F, t6335: F, t9483: F) -> (F, F, F, F, F, F, F) {
    let t142503 = t7581 * t25491;
    let t142512 = t1466 * t24986;
    let t142527 = t1466 * t2336 * t34290;
    let t142537 = t342 * t2252 * t7574 / 18.0;
    let t142539 = t342 * t630 * t34305;
    let t142558 = t7570 * t1774 * t6343;
    let t142566 = t1526 * t9483 * t6335;
    (t142503, t142512, t142527, t142537, t142539, t142558, t142566)
}
