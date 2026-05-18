//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1250/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1250<F: Float>(t32833: F, t32834: F, t32839: F, t35713: F, t37658: F, t37661: F, t37663: F, t37665: F, t37672: F, t37675: F, t40105: F, t40107: F, t40109: F, t40111: F, t40114: F, t40118: F, t40121: F, t40123: F) -> F {
    let t41948 = -t37658 + F::new(0.34299214494455789578e-1) * t40105 + t37661 - F::new(0.34299214494455789578e-1) * t40107 - F::new(0.34299214494455789578e-2) * t40109 - F::new(0.34299214494455789578e-2) * t40111 - t37663 - t37665 + F::new(0.85748036236139473944e-3) * t40114 + t37672 + F::new(0.31448092289604152068e-2) * t40118 - t32833 - t32834 - t37675 - t35713 - t32839 - F::new(7.0) / F::new(24.0) * t40121 - t40123 / F::new(24.0);
    t41948
}
