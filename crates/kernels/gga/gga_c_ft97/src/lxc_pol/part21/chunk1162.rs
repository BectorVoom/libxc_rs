//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1162/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1162<F: Float>(t22952: F, t22953: F, t29711: F, t379: F, t101693: F, t3188: F, t5674: F, t29688: F, t376: F, t89: F, t102151: F, t102164: F, t102165: F, t116509: F, t116512: F, t116515: F, t116518: F, t116521: F, t116526: F) -> (F, F, F, F, F) {
    let t116530 = t22952 * t22953 * t29711 * t379;
    let t116532 = t101693 * t3188;
    let t116534 = t5674 * t22953 * t116532;
    let t116537 = t89 * t376 * t29688;
    let t116538 = 2.0 * t116537;
    let t116539 = t102151 + t116509 + t116512 / 3.0 + 2.0 * t116515 + 4.0 / 3.0 * t116518 - 4.0 / 9.0 * t116521 - t116526 / 12.0 - t116530 / 12.0 - 2.0 / 3.0 * t116534 + t116538 + t102164 + t102165;
    (t116530, t116532, t116534, t116537, t116539)
}
