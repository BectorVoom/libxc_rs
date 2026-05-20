//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1905/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1905<F: Float>(t2435: F, t8099: F, t25904: F, t26231: F, t97802: F, t26234: F, t98041: F, t102244: F, t94674: F, t97700: F, t102268: F, t102165: F) -> (F, F, F, F, F, F, F, F) {
    let t102315 = t8099 * t2435;
    let t102316 = t25904 * t102315;
    let t102320 = F::cast_from(0.14456046980341999104e-1_f64) * t97802 * t26231;
    let t102324 = F::cast_from(0.51405703062096148812e-1_f64) * t98041 * t26234;
    let t102325 = t94674 * t102244;
    let t102329 = F::cast_from(0.28912093960683998208e-1_f64) * t97700 * t26234;
    let t102339 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t102268;
    let t102346 = F::cast_from(0.14456046980341999104e-1_f64) * t25904 * t102165;
    (t102315, t102316, t102320, t102324, t102325, t102329, t102339, t102346)
}
