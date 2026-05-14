//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1120/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1120<F: Float>(t1972: F, t730: F, t7527: F, t1987: F, t7571: F, t1957: F, t7535: F, t1116: F, t17312: F, t21186: F, t21196: F, t21217: F, t21220: F, t21223: F, t21225: F, t21233: F) -> (F, F, F, F, F) {
    let t21313 = 0.35089341735807877242e1 * t730 * t7527 * t1972;
    let t21315 = 0.10526802520742363173e2 * t1987 * t7571;
    let t21318 = 0.10526802520742363173e2 * t730 * t7535 * t1957;
    let t21320 = 0.5848223622634646207e0 * t17312 * t1116;
    let t21321 = t21313 - t21315 - t21318 + t21186 - t21196 + t21217 + t21220 + t21223 + t21225 - t21320 + t21233;
    (t21313, t21315, t21318, t21320, t21321)
}
