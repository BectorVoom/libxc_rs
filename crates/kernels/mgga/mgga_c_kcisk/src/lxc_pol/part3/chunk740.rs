//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 740/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk740<F: Float>(t10777: F, t11252: F, t11255: F, t11257: F, t11261: F, t11265: F, t11271: F, t11275: F, t11281: F, t11322: F, t11392: F, t11448: F, t1421: F, t1689: F, t4794: F, t604: F) -> F {
    let t11450 = -F::new(12.0) * t1689 * t4794 + t11252 - F::new(4.0) * t604 * t10777 + F::new(0.39422578e-2) * t11255 - F::new(0.26281718666666666667e-2) * t11257 + F::new(0.39422577999999999999e-2) * t1421 * t11261 + F::new(0.39422577999999999999e-2) * t1421 * t11265 + F::new(0.1478346675e-2) * t1421 * t11271 - F::new(0.59133867e-2) * t1421 * t11275 + F::new(0.49278222499999999999e-2) * t1421 * t11281 + t11322 + t11392 + t11448;
    t11450
}
