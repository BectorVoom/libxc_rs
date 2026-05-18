//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1430/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1430<F: Float>(t33407: F, t36570: F, t36571: F, t36572: F, t36573: F, t36574: F, t36575: F, t36577: F, t36578: F, t36579: F, t36580: F, t33464: F, t33474: F, t36596: F, t36597: F, t36599: F, t36600: F, t36601: F, t36602: F, t36604: F, t36605: F, t36606: F) -> (F, F) {
    let t38728 = -t36570 + t36571 - t36572 - t36573 + t36574 - t36575 - F::new(0.36231816839129402172e-6) * t33407 + t36577 + t36578 + t36579 - t36580;
    let t38740 = -t36596 - t36597 - F::new(0.18115908419564701086e-6) * t33464 + t36599 - t36600 + t36601 + t36602 - F::new(0.56912804804009946682e-7) * t33474 - t36604 + t36605 + t36606;
    (t38728, t38740)
}
