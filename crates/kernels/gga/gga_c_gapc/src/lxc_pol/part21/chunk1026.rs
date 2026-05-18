//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1026/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1026<F: Float>(t640: F, t6939: F, t102: F, t2404: F, t2446: F, t2254: F, t830: F, t122: F, t6924: F, t2299: F, t2530: F, t6851: F, t768: F) -> (F, F, F, F, F, F) {
    let t23305 = t6939 * t640;
    let t23343 = t2446 * t102 * t2404;
    let t23466 = t830 * t2254;
    let t23523 = t6924 * t122;
    let t23579 = t2530 * t102 * t2299;
    let t23608 = t768 * t6851;
    (t23305, t23343, t23466, t23523, t23579, t23608)
}
