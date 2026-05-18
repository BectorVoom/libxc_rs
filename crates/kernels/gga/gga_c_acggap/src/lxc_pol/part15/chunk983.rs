//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 983/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk983<F: Float>(t2268: F, t30797: F, t30543: F, t8473: F, t31419: F, t4810: F, t721: F, t30673: F, t1503: F, t7329: F, t1992: F, t5616: F, t7585: F, t7586: F) -> (F, F, F, F, F, F) {
    let t34638 = t30797 * t2268;
    let t34640 = t30543 * t8473;
    let t34650 = t31419 * t4810 * t721;
    let t34655 = F::new(0.34299214494455789578e-2) * t30673;
    let t34659 = t7329 * t1503;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    (t34638, t34640, t34650, t34655, t34659, t34675)
}
