//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1406/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1406<F: Float>(t25253: F, t5393: F, t114726: F, t114728: F, t114734: F, t114747: F, t114749: F, t114757: F, t1212: F, t125534: F, t126846: F, t127889: F, t128334: F, t193: F, t2862: F, t28843: F, t29369: F, t295: F, t296: F, t312: F, t31551: F, t319: F, t4246: F, t446: F, t5225: F, t6393: F, t840: F, t871: F, t875: F, t89: F) -> (F, F) {
    let t128362 = t25253 * t5393;
    let t128370 = t114726 + 2.0 / 3.0 * t446 * t2862 * t6393 * t5225 + t89 * t193 * t295 * t128334 * t312 / 3.0 - t114728 - t114734 + 2.0 / 3.0 * t446 * t2862 * t319 * t127889 + 2.0 / 3.0 * t446 * t2862 * t319 * t126846 + t446 * t840 * t871 * t31551 * t875 / 3.0 + 2.0 / 3.0 * t446 * t840 * t4246 * t29369 - t114747 + 8.0 / 27.0 * t114749 - 2.0 / 3.0 * t446 * t840 * t28843 * t1212 - t446 * t296 * t128362 / 3.0 - 2.0 / 3.0 * t446 * t296 * t125534 - 8.0 / 81.0 * t114757;
    (t128362, t128370)
}
