//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2174/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2174<F: Float>(t25375: F, t99365: F, t1579: F, t25392: F, t4481: F, t92921: F, t10073: F, t1958: F, t25390: F, t25305: F, t99380: F, t213: F, t27265: F) -> (F, F, F, F, F, F) {
    let t99412 = t25375 * t99365;
    let t99414 = t25392 * t1579;
    let t99420 = F::cast_from(0.19514881078765566038e-1_f64) * t92921 * t4481;
    let t99423 = t10073 * t25390 * t1958 * t1579;
    let t99425 = t25305 * t99380;
    let t99429 = t213 * t27265;
    (t99412, t99414, t99420, t99423, t99425, t99429)
}
