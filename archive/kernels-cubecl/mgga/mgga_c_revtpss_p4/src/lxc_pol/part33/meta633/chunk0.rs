//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2080/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2080<F: Float>(t25431: F, t99389: F, t1568: F, t786: F, t25410: F, t25413: F, t25375: F, t99365: F, t1579: F, t25392: F, t4481: F, t92921: F) -> (F, F, F, F, F, F, F) {
    let t99391 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t99389;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99406 = F::cast_from(0.14456046980341999104e-1_f64) * t99404 * t25413;
    let t99412 = t25375 * t99365;
    let t99414 = t25392 * t1579;
    let t99420 = F::cast_from(0.19514881078765566038e-1_f64) * t92921 * t4481;
    (t99391, t99403, t99404, t99406, t99412, t99414, t99420)
}
