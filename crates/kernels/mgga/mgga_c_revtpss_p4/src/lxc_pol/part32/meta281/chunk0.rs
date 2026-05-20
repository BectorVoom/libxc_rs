//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1177/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1177<F: Float>(t1333: F, t3860: F, t30: F, t513: F, t33: F, t516: F, t2435: F, t3900: F, t3896: F, t9303: F, t1419: F, t785: F) -> (F, F, F, F, F, F) {
    let t9597 = t3860 * t1333;
    let t9603 = t30 * t30;
    let t9605 = F::new(1.0) / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = F::new(1.0) / t516 / t9615;
    let t9632 = t2435 * t3900;
    let t9639 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t3896;
    let t9640 = t785 * t1419;
    (t9597, t9605, t9617, t9632, t9639, t9640)
}
