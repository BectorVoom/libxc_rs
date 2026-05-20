//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1601/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1601<F: Float>(t30: F, t1450: F, t6922: F, t6785: F, t9605: F, t3874: F, t5824: F, t1344: F, t18280: F, t2255: F, t5574: F, t605: F, t6792: F, t9617: F, zeta_threshold: F) -> (F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t21937 = t6922 * t1450;
    let t21944 = t9605 * t6785;
    let t21949 = t3874 * t5824;
    let t21955 = piecewise3::<F>(t31, F::new(0.0), F::new(8.0) / F::new(27.0) * t21944 * t605 - F::new(8.0) / F::new(9.0) * t5574 * t2255 - F::new(2.0) / F::new(9.0) * t21949 * t605 + F::new(2.0) / F::new(3.0) * t1344 * t18280);
    let t21956 = t9617 * t6792;
    (t21937, t21955, t21956)
}
