//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2209/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2209<F: Float>(t2126: F, t2371: F, t13514: F, t1519: F, t2163: F, t2322: F, t2328: F, t27060: F, t29337: F, t29432: F, t29459: F, t4257: F, t4293: F, t651: F, t670: F, t8233: F, t97610: F, t97617: F, t97629: F, t97639: F, t97641: F, t97643: F, t97645: F, t97647: F, t97649: F, t97653: F, t97657: F, t97659: F) -> (F, F) {
    let t104138 = t2126 * t2371;
    let t104153 = -F::cast_from(2.0_f64) * t13514 * t2163 * t651 - F::cast_from(4.0_f64) * t29337 * t651 * t670 - F::cast_from(2.0_f64) * t104138 * t1519 - F::cast_from(4.0_f64) * t2322 * t29459 - F::cast_from(2.0_f64) * t2328 * t8233 - F::cast_from(4.0_f64) * t27060 * t4293 - F::cast_from(4.0_f64) * t29432 * t4257 - t97610 - t97617 - t97629 - t97639 - t97641 - t97643 - t97645 - t97647 - t97649 + t97653 + t97657 + t97659;
    (t104138, t104153)
}
