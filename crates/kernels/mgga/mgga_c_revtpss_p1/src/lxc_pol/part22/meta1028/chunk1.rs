//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3605/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t68429 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t68297 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t68301 + F::cast_from(2.0_f64) * t68305 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t68310 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t68332 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t68334 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t68336 + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t68342 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t68347 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t68350 - F::cast_from(8.0_f64) * t68353 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t68357 + F::cast_from(8.0_f64) * t68360;
    t68429
}
