//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 618/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk618<F: Float>(t1204: F, t1284: F, t487: F, t1209: F, t1269: F, t473: F, t3140: F, t3596: F) -> (F, F, F, F, F) {
    let t3746 = t1204 * t1284;
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3759 = t473 * t1269;
    let t3766 = t3140 * t3596;
    (t3746, t3754, t3755, t3759, t3766)
}
