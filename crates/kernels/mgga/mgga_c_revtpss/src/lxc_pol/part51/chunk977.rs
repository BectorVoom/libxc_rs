//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 977/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk977<F: Float>(t32366: F, t572: F, t7002: F, t7330: F, t1459: F, t8614: F, t116: F, t8460: F, t670: F, t1461: F, t32354: F, t32358: F, t32360: F, t32362: F, t32365: F, t573: F, t8607: F, t8616: F) -> (F, F, F, F, F, F) {
    let t32368 = F::new(6.0) * t572 * t32366;
    let t32369 = t7330 * t7002;
    let t32371 = F::new(12.0) * t572 * t32369;
    let t32372 = t1459 * t8614;
    let t32373 = F::new(3.0) * t32372;
    let t32374 = t116 * t8460;
    let t32375 = t32374 * t670;
    let t32376 = t572 * t32375;
    let t32377 = F::new(6.0) * t32376;
    let t32378 = F::new(3.0) * t1461 * t8607 + t32354 * t573 + F::new(6.0) * t32358 + F::new(12.0) * t32360 + F::new(6.0) * t32362 + t32365 + t32368 + t32371 + t32373 + t32377 + t8616;
    (t32369, t32373, t32374, t32375, t32377, t32378)
}
