//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1369/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1369<F: Float>(t21804: F, t77: F, t1471: F, t1487: F, t1494: F, t21727: F, t21769: F, t4188: F, t4191: F, t4196: F, t4218: F, t4238: F, t5855: F, t5869: F, t608: F, t628: F, t641: F, t71: F, t85: F) -> F {
    let t21805 = t77 * t21804;
    let t21808 = -t4188 * t1494 / F::new(6.0) - t4191 * t1494 / F::new(6.0) - t1471 * t4238 / F::new(6.0) - t21727 * t85 / F::new(12.0) + t21769 * t85 / F::new(24.0) + t5855 * t641 / F::new(24.0) - t4196 * t1494 / F::new(6.0) + t4218 * t1494 / F::new(12.0) + t1487 * t4238 / F::new(12.0) - t608 * t5869 / F::new(12.0) + t628 * t5869 / F::new(24.0) + t71 * t21805 / F::new(24.0);
    t21808
}
