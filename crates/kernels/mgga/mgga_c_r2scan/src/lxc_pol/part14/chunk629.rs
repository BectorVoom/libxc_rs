//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 629/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk629<F: Float>(t322: F, t1120: F, t829: F, t1300: F, t327: F, t3506: F, t3509: F, t834: F, t330: F, t1125: F, t837: F, t3505: F) -> (F, F, F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t3512 = t1120 * t829;
    let t3517 = -F::new(0.64e0) * t3506 * t327 - F::new(0.128e1) * t3509 * t829 - F::new(0.128e1) * t1300 * t3512 - F::new(0.64e0) * t834 * t3506;
    let t3518 = t3517 * t330;
    let t3519 = t1125 * t837;
    let t3520 = t3519 * t330;
    let t3522 = piecewise3::<F>(t332, F::new(0.0), t3505);
    (t3517, t3518, t3520, t3522)
}
