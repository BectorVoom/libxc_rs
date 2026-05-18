//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 572/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk572<F: Float>(t1282: F, t1291: F, t187: F, t3324: F, t3327: F, t3333: F, t3482: F, t3662: F, t3664: F, t3669: F, t3670: F, t3699: F, t437: F) -> F {
    let t3703 = t3324 - t3327 + t3333 - t3482 + t187 * (-t1282 * t3699 - F::new(2.0) * t1291 * t3664 + t3662 * t437 + F::new(2.0) * t3669 * t3670 - t3324 + t3327 - t3333 + t3482);
    t3703
}
