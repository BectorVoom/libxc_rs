//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1280/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1280<F: Float>(t5: F, t130845: F, t130895: F, t117: F, t127532: F, t127545: F, t127547: F, t127549: F, t127550: F, t127556: F, t127559: F, t128195: F, t1310: F, t33578: F, t33580: F, t33583: F, t34776: F, t508: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t130897 = piecewise3::<f64>(t8, F::new(0.0), t130845 + t130895);
    let t130898 = t130897 * t117;
    let t130901 = -t130898 * t508 - t1310 * t34776 + t127532 - t127545 - t127547 - t127549 - t127550 - t127556 + t127559 - t128195 - t33578 - t33580 - t33583;
    (t130898, t130901)
}
