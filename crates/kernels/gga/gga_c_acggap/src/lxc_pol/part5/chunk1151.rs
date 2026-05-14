//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1151/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1151<F: Float>(t1140: F, t6171: F, t6175: F, t1805: F, t3570: F, t3379: F, t6237: F, t3409: F, t5899: F, t1096: F, t1165: F, t1173: F, t1181: F, t18388: F, t18392: F, t18396: F, t18400: F, t22705: F, t3396: F, t4267: F, t4463: F, t4533: F, t4706: F, t5852: F) -> (F,) {
    let t23751 = t1140 * t6171;
    let t23753 = t1140 * t6175;
    let t23755 = t3570 * t1805;
    let t23765 = t3379 * t6237;
    let t23773 = t3409 * t5899;
    let t23777 = 7.0 / 72.0 * t23751 + 7.0 / 72.0 * t23753 - 35.0 / 216.0 * t23755 + 0.68598428988911579156e-2 * t3396 * t1181 * t22705 * t1096 - 0.17149607247227894789e-1 * t4463 * t1165 * t4267 * t4533 + 0.17149607247227894789e-2 * t23765 + 0.85748036236139473944e-3 * t1173 * t1165 * t5852 * t4706 - 0.10289764348336736873e-1 * t18388 - 0.51448821741683684366e-2 * t18392 - 0.40015750243531754508e-2 * t23773 - 0.25724410870841842183e-2 * t18396 - 0.51448821741683684366e-2 * t18400;
    (t23777,)
}
