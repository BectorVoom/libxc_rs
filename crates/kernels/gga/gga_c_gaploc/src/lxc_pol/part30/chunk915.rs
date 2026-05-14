//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 915/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk915<F: Float>(t1024: F, t2717: F, t2508: F, t2927: F, t954: F, t3464: F, t702: F, t3433: F, t779: F, t3431: F, t835: F) -> (F, F, F, F, F, F, F) {
    let t10770 = t2717 * t1024;
    let t10772 = 0.76905262301422242837e-2 * t2508 * t10770;
    let t10773 = t954 * t2927;
    let t10775 = 0.76905262301422242837e-2 * t2508 * t10773;
    let t10776 = t3464 * t702;
    let t10779 = t779 * t3433;
    let t10782 = t835 * t3431;
    (t10770, t10772, t10773, t10775, t10776, t10779, t10782)
}
