//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 779/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk779<F: Float>(t28983: F, t959: F, t28846: F, t12696: F, t5676: F, t2033: F, t2365: F, t2610: F, t9688: F, t12695: F, t549: F, t12692: F, t2013: F, t10007: F, t2530: F, t825: F, t9438: F) -> (F, F, F, F, F, F, F) {
    let t41281 = t28983 * t959;
    let t41283 = t28846 * t959;
    let t41286 = t5676 * t12696;
    let t41290 = t2033 * t2365 * t2610 * t9688;
    let t41293 = t2033 * t549 * t12695;
    let t41295 = t2013 * t12692;
    let t41299 = t825 * t9438 * t10007 * t2530;
    (t41281, t41283, t41286, t41290, t41293, t41295, t41299)
}
