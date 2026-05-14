//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1149/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1149<F: Float>(t3379: F, t5623: F, t1165: F, t14174: F, t18347: F, t18349: F, t18351: F, t18364: F, t18366: F, t21532: F, t23676: F, t23680: F, t23682: F, t23686: F, t23691: F, t3176: F, t3403: F) -> (F,) {
    let t23697 = t3379 * t5623;
    let t23702 = 0.32012600194825403606e-1 * t18347 + 0.25724410870841842184e-2 * t23676 + 0.16006300097412701803e-1 * t18349 + 0.80031500487063509016e-2 * t18351 + 0.12004725073059526353e-1 * t23680 - 0.56688979511669985553e-2 * t23682 - 0.85748036236139473944e-3 * t23686 + 0.85748036236139473944e-3 * t23691 + 0.17149607247227894789e-1 * t3403 * t1165 * t21532 * t3176 - 0.68598428988911579156e-2 * t23697 + 0.34299214494455789578e-2 * t14174 - 7.0 / 36.0 * t18364 - 7.0 / 36.0 * t18366;
    (t23702,)
}
