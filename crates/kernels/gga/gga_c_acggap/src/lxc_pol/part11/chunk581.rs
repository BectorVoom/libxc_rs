//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 581/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk581<F: Float>(t1165: F, t1586: F, t407: F, t1562: F, t3379: F, t1567: F, t1466: F, t3382: F, t157: F, t839: F, t1532: F, t1077: F, t1181: F, t1545: F, t3431: F, t1524: F, t322: F) -> (F, F, F, F, F, F, F, F) {
    let t4695 = t1165 * t1586 * t407;
    let t4699 = 0.17149607247227894789e-2 * t3379 * t1562;
    let t4701 = t1165 * t1567 * t407;
    let t4705 = 0.85748036236139473944e-3 * t3382 * t1466;
    let t4706 = t157 * t839;
    let t4708 = t1165 * t1532 * t4706;
    let t4711 = t157 * t1077;
    let t4712 = t1532 * t4711;
    let t4713 = t1181 * t4712;
    let t4716 = t3431 * t1545;
    let t4718 = t1524 * t322;
    (t4695, t4699, t4701, t4705, t4708, t4713, t4716, t4718)
}
