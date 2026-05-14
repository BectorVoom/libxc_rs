//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1017/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1017<F: Float>(t1008: F, t6110: F, t1005: F, t5971: F, t1089: F, t175: F, t322: F, t384: F, t5506: F, t1734: F, t879: F, t5826: F, t12421: F, t1298: F, t1345: F, t1427: F, t146: F, t1487: F, t15337: F, t15814: F, t368: F, t4099: F, t418: F, t4255: F, t4256: F, t4875: F, t495: F, t506: F) -> (F,) {
    let t20238 = t1008 * t6110;
    let t20263 = t1005 * t5971;
    let t20268 = t384 * t1089 * t175 * t5506 * t322;
    let t20273 = t384 * t1089 * t175 * t1734 * t879;
    let t20275 = t1005 * t5826;
    let t20278 = -0.17149607247227894789e-2 * t20238 - t15814 * t146 * t1427 * t1298 - t4255 * t4256 * t1345 * t1298 / 4.0 + 0.24009450146119052704e-1 * t15337 - 0.34299214494455789578e-2 * t418 * t1089 * t368 * t4099 * t506 - 0.68598428988911579156e-2 * t418 * t1089 * t368 * t1298 * t1487 - 0.34299214494455789578e-2 * t418 * t1089 * t368 * t495 * t4875 + 0.17149607247227894789e-2 * t20263 + 0.17149607247227894789e-2 * t20268 + 0.85748036236139473944e-3 * t20273 - 0.85748036236139473944e-2 * t20275 - 0.24009450146119052705e-1 * t12421;
    (t20278,)
}
