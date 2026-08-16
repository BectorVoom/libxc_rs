//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1891/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1891<F: Float>(t25823: F, t4287: F, t116: F, t28683: F, t101453: F, t26179: F, t28133: F, t7706: F, t95293: F, t60224: F, t7342: F, t13272: F, t26178: F) -> (F, F, F, F, F, F, F) {
    let t101455 = t25823 * t4287;
    let t101705 = t116 * t28683;
    let t101755 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t101453;
    let t101756 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t101455;
    let t101782 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t26179 * t28133;
    let t101783 = t95293 * t7706;
    let t101785 = t60224 * t7342;
    let t101788 = t13272 * t26178;
    (t101705, t101755, t101756, t101782, t101783, t101785, t101788)
}
