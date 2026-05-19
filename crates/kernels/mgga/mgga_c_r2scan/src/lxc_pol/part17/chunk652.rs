//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 652/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk652<F: Float>(t322: F, t352: F, t3549: F, t3556: F, t3675: F, t3741: F, t3743: F, t3771: F, t3774: F, t855: F, t3564: F, t3565: F, t3566: F, t3567: F, t3690: F, t3694: F, t3702: F, t3704: F, t3707: F, t3722: F, t3725: F) -> (F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t3781 = piecewise5::<F>(t323, t3741 + t3743, t331, t3771, -F::new(0.21e1) * t3549 * t3675 - F::new(0.105e1) * t855 * t3774 * t352 - F::new(0.1575e1) * t3556 * t3675);
    let t3787 = -t3564 + t3565 - t3566 - t3567 - F::cast_from(0.72042316457491791901e-3_f64) * t3690 + F::cast_from(0.30487649791575028312e-3_f64) * t3694 - t3702 - t3704 + t3707 - t3722 + t3725;
    (t3781, t3787)
}
