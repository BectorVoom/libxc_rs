//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3648/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t68997 = F::cast_from(0.37083333333333333334e-1_f64) * t68297 + F::cast_from(0.18541666666666666667e-1_f64) * t68301 + F::cast_from(0.55625000000000000001e-1_f64) * t68305 - F::cast_from(0.27469135802469135803e-1_f64) * t68310 + F::cast_from(0.41203703703703703704e-2_f64) * t68332 + F::cast_from(0.82407407407407407407e-2_f64) * t68334 + F::cast_from(0.24722222222222222222e-1_f64) * t68336 + F::cast_from(0.10300925925925925926e-1_f64) * t68342 + F::cast_from(0.12361111111111111111e0_f64) * t68347 - F::cast_from(0.37083333333333333333e-1_f64) * t68350 - F::cast_from(0.22249999999999999999e0_f64) * t68353 - F::cast_from(0.12361111111111111111e-1_f64) * t68357 + F::new(0.2225e0) * t68360;
    t68997
}
