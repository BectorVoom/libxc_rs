//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 766/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk766<F: Float>(t154: F, t1885: F, t5663: F, t276: F, t465: F, t779: F, t179: F, t655: F, t299: F, t2054: F, t2057: F, t2082: F, t2091: F, t5630: F, t5637: F, t5641: F, t5646: F, t5649: F, t5658: F, t5661: F, t735: F, t744: F, t782: F) -> (F, F, F, F, F, F) {
    let t5665 = t154 * t5663 * t1885;
    let t5666 = t276 * t5665;
    let t5672 = t465 * t779;
    let t5674 = t179 * t5672 * t655;
    let t5675 = t299 * t5674;
    let t5677 = 0.25724410870841842184e-2 * t5630 - 0.51448821741683684368e-2 * t299 * t5637 - 0.42874018118069736972e-3 * t299 * t5641 - t5646 / 96.0 - t276 * t5649 / 96.0 - 11.0 / 36.0 * t2057 * t744 - t735 * t2091 / 6.0 - t276 * t5658 / 16.0 + t5661 / 18.0 + t5666 / 48.0 + t735 * t2054 / 12.0 - 0.43445671692977333464e-1 * t2082 * t782 + 0.28582678745379824648e-3 * t5675;
    (t5665, t5666, t5672, t5674, t5675, t5677)
}
