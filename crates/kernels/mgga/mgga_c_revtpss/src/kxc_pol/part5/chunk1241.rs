//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1241/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1241<F: Float>(t1470: F, t4217: F, t1494: F, t21686: F, t21687: F, t21690: F, t21695: F, t21699: F, t21702: F, t21707: F, t21710: F, t4182: F, t5820: F, t5827: F, t5830: F, t641: F, t85: F) -> (F,) {
    let t21713 = t1470 * t4217;
    let t21720 = -t21686 * t21687 / 6.0 - t21690 * t85 / 12.0 - t5820 * t641 / 12.0 - t21695 * t85 / 12.0 - t21699 * t85 / 12.0 - t21702 * t85 / 12.0 - t5827 * t641 / 12.0 - t21707 * t85 / 6.0 - t21710 * t85 / 6.0 - t21713 * t85 / 6.0 - t5830 * t641 / 6.0 - t4182 * t1494 / 6.0;
    (t21720,)
}
