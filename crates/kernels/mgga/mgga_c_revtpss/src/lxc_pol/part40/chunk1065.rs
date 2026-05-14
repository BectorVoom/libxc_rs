//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1065/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1065<F: Float>(t30: F, t33: F, t2: F, t3874: F, t1344: F, t13554: F, t13687: F, t22: F, t2257: F, t3834: F, t5574: F, t5577: F, t580: F, t1711: F, t9617: F, t3881: F, t1348: F, t13569: F, t3351: F, t3842: F, t5582: F, t5585: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13690 = t3874 * t2;
    let t13700 = piecewise3(t31, 0.0, 8.0 / 27.0 * t13687 * t3834 - 8.0 / 9.0 * t13690 * t13554 - 2.0 / 9.0 * t5574 * t2257 + 4.0 / 3.0 * t1344 * t580 - 4.0 * t5577 * t22);
    let t13701 = t9617 * t1711;
    let t13704 = t3881 * t2;
    let t13714 = piecewise3(t34, 0.0, 8.0 / 27.0 * t13701 * t3842 + 8.0 / 9.0 * t13704 * t13569 - 2.0 / 9.0 * t5582 * t3351 - 4.0 / 3.0 * t1348 * t580 + 4.0 * t5585 * t22);
    (t13700, t13714)
}
