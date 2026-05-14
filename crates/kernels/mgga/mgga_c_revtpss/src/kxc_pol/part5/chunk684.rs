//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 684/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk684<F: Float>(t2848: F, t2906: F, t2950: F, t2957: F, t4571: F, t4576: F, t4581: F, t4585: F, t4599: F, t4607: F, t4615: F, t4617: F, t4620: F, t4623: F, t4626: F, t4629: F) -> (F,) {
    let t4669 = -0.17648625e1 * t4599 + 0.3529725e1 * t4607 + t2950 + 0.17215833333333333333e0 * t2848 + 0.17215833333333333333e0 * t4571 - 0.34431666666666666667e0 * t4576 + 0.103295e1 * t4581 - 0.516475e0 * t4585 + 0.31558125e0 * t4615 + 0.6311625e0 * t4617 + t2957 + 0.69463333333333333333e-1 * t2906 + 0.69463333333333333333e-1 * t4620 - 0.34731666666666666667e-1 * t4623 + 0.20839e0 * t4626 - 0.104195e0 * t4629;
    (t4669,)
}
