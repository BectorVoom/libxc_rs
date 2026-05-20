//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2877/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2877<F: Float>(t23421: F, t2411: F, t1940: F, t23429: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t41154: F, t76955: F, t76957: F, t76960: F, t890: F) -> F {
    let t77357 = t23421 * t2411;
    let t77360 = -F::new(6.0) * t1940 * t23429 * t41154 * t890 - t1940 * t77357 * t890 + t39520 - t39528 + t39531 + t39534 + t39537 - t39540 + t76955 + t76957 + t76960;
    t77360
}
