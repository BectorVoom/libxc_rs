//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1327/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1327<F: Float>(t30: F, t5566: F, t749: F, t512: F, t9856: F, t1468: F, t9605: F, t2: F, t3874: F, t1344: F, t13554: F, t22: F, t2257: F, t3834: F, t5574: F, t5577: F, t580: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t13680 = t5566 * t749;
    let t13682 = F::new(2.0) * t512 * t13680;
    let t13683 = F::new(48.0) * t9856;
    let t13687 = t9605 * t1468;
    let t13690 = t3874 * t2;
    let t13700 = piecewise3::<F>(t31, F::new(0.0), F::new(8.0) / F::new(27.0) * t13687 * t3834 - F::new(8.0) / F::new(9.0) * t13690 * t13554 - F::new(2.0) / F::new(9.0) * t5574 * t2257 + F::new(4.0) / F::new(3.0) * t1344 * t580 - F::new(4.0) * t5577 * t22);
    (t13682, t13683, t13700)
}
