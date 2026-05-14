//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 671/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk671<F: Float>(t21502: F, t28668: F, t2012: F, t7809: F, t2530: F, t299: F, t1890: F, t27997: F, t7802: F, t296: F, t9688: F, t1: F, t787: F, t5638: F, t822: F, t9419: F) -> (F, F, F, F, F, F, F) {
    let t28669 = t21502 * t28668;
    let t28673 = t2012 * t7809;
    let t28703 = t299 * t2530;
    let t28720 = t1890 * t27997;
    let t28737 = t2012 * t7802;
    let t28844 = t296 * t9688;
    let t28846 = t787 * t28844 * t1;
    let t28856 = t822 * t5638 * t9419;
    (t28669, t28673, t28703, t28720, t28737, t28846, t28856)
}
