//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 850/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk850<F: Float>(t1960: F, t2728: F, t3684: F, t11711: F, t23555: F, t10298: F, t8045: F, t2902: F, t3366: F, t4349: F, t11701: F, t11556: F, t2355: F) -> (F, F, F, F, F, F) {
    let t45144 = F::new(2.0) * t1960 * t3684 * t2728;
    let t45146 = F::new(6.0) * t23555 * t11711;
    let t45148 = F::new(4.0) * t8045 * t10298;
    let t45151 = F::new(12.0) * t4349 * t3366 * t2902;
    let t45163 = t11701 * t2728;
    let t45164 = t2355 * t11556;
    (t45144, t45146, t45148, t45151, t45163, t45164)
}
