//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 751/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk751<F: Float>(t103: F, t2368: F, t2376: F, t645: F, t66: F, t237: F, t663: F, t2303: F, t2334: F, t2338: F, t88: F, t2342: F) -> (F, F, F, F, F) {
    let t8637 = t103 * t2368;
    let t8640 = t103 * t2376;
    let t8643 = t66 * t645;
    let t8646 = F::new(0.71233333333333333334e-1) * t237 * t8643 * t663;
    let t8649 = F::new(0.53425e-1) * t237 * t2303 * t2334;
    let t8650 = t88 * t2338;
    let t8653 = F::new(0.85917146441092277512e0) * t237 * t8650 * t2342;
    (t8637, t8640, t8646, t8649, t8653)
}
