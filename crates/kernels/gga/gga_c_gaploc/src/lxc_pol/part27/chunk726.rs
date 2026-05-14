//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 726/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk726<F: Float>(t701: F, t7221: F, t1901: F, t1835: F, t2571: F, t2530: F, t835: F, t723: F, t2580: F, t161: F, t2536: F, t1854: F, t296: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7250 = t7221 * t701;
    let t7251 = t1901 * t7250;
    let t7254 = t2571 * t1835;
    let t7255 = t1901 * t7254;
    let t7258 = t835 * t2530;
    let t7259 = t7258 * t723;
    let t7260 = t2580 * t7259;
    let t7267 = t2536 * t161;
    let t7268 = t7267 * t1854;
    let t7275 = t296 * t2530;
    (t7250, t7251, t7254, t7255, t7258, t7259, t7260, t7268, t7275)
}
