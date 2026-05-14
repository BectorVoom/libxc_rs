//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 906/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk906<F: Float>(t5213: F, t7822: F, t157: F, t33750: F, t1165: F, t2068: F, t604: F, t30230: F, t30233: F, t30234: F, t30239: F, t30243: F, t30247: F, t30249: F, t30251: F, t30253: F, t33956: F, t33960: F, t33963: F, t33966: F, t33968: F, t33970: F) -> (F, F) {
    let t33974 = t7822 * t5213;
    let t33976 = t33750 * t157;
    let t33979 = t2068 * t1165 * t604 * t33976;
    let t33981 = -0.21437009059034868486e-2 * t33956 - 0.38203125e-2 * t33960 + t33963 - t33966 / 128.0 + t33968 + t30230 + t30233 - 0.85748036236139473944e-3 * t30234 + t30239 + t33970 + t30243 - t30247 - 0.90702367218671976886e-1 * t30249 - 0.12004725073059526352e-1 * t30251 + 0.85748036236139473945e-2 * t30253 - 0.17149607247227894789e-2 * t33974 + 0.15724046144802076034e-3 * t33979;
    (t33976, t33981)
}
