//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 597/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk597<F: Float>(t1625: F, t5802: F, t327: F, t5420: F, t1637: F, t5755: F, t1240: F, t1610: F, t93: F, t5123: F, t328: F, t5759: F, t1579: F, t305: F, t1519: F, t304: F) -> (F, F, F, F, F, F, F, F) {
    let t5803 = t5802 * t1625;
    let t5805 = t327 * t5420;
    let t5806 = t5805 * t1625;
    let t5812 = t1637 * t5755 / 6.0;
    let t5813 = t1610 * t1240;
    let t5814 = t93 * t5813;
    let t5815 = t5123 * t5814;
    let t5817 = t328 * t5759;
    let t5819 = t1579 * t305;
    let t5829 = t304 * t1519;
    (t5803, t5806, t5812, t5814, t5815, t5817, t5819, t5829)
}
