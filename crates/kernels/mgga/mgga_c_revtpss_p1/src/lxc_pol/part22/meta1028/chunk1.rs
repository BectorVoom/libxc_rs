//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3605/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t68429 = F::new(4.0) / F::new(3.0) * t68297 + F::new(2.0) / F::new(3.0) * t68301 + F::new(2.0) * t68305 - F::new(80.0) / F::new(81.0) * t68310 + F::new(4.0) / F::new(27.0) * t68332 + F::new(8.0) / F::new(27.0) * t68334 + F::new(8.0) / F::new(9.0) * t68336 + F::new(10.0) / F::new(27.0) * t68342 + F::new(40.0) / F::new(9.0) * t68347 - F::new(4.0) / F::new(3.0) * t68350 - F::new(8.0) * t68353 - F::new(4.0) / F::new(9.0) * t68357 + F::new(8.0) * t68360;
    t68429
}
