//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1367/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1367<F: Float>(t2647: F, t695: F, t113124: F, t5492: F, t117840: F, t9740: F, t112982: F, t5509: F, t7261: F, t117898: F, t2804: F, t112904: F, t2642: F, t5437: F, t33167: F, t34412: F) -> (F, F, F, F, F, F, F) {
    let t118011 = t2647 * t695;
    let t118013 = t113124 * t118011 * t5492;
    let t118021 = 0.11574074074074074074e-2 * t9740 * t117840;
    let t118028 = t7261 * t112982 * t2647 * t5509;
    let t118032 = 0.34722222222222222222e-2 * t2804 * t117898;
    let t118037 = t7261 * t112904 * t2642 * t5437;
    let t118040 = t34412 * t33167;
    (t118011, t118013, t118021, t118028, t118032, t118037, t118040)
}
