//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 573/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk573<F: Float>(t642: F, t695: F, t1060: F, t1757: F, t5192: F, t5182: F, t1801: F, t4644: F, t1800: F, t1799: F, t1755: F, t654: F, t4972: F, t1869: F, t1693: F, t4827: F, t5057: F, t5066: F, t5071: F, t5075: F, t5078: F, t5080: F, t5178: F, t5189: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5193 = t642 * t695;
    let t5194 = t1060 * t1757;
    let t5195 = t5193 * t5194;
    let t5196 = t5192 * t5195;
    let t5197 = t5182 * t5196;
    let t5199 = t1801 * t4644;
    let t5200 = t1800 * t5199;
    let t5201 = t1799 * t5200;
    let t5203 = t654 * t1755;
    let t5204 = t5203 * t4972;
    let t5205 = t1800 * t5204;
    let t5206 = t1869 * t5205;
    let t5210 = 0.27636574074074074073e-2 * t5057 + 0.49745833333333333332e-2 * t5066 - 0.33163888888888888888e-2 * t5071 + 0.22109259259259259258e-2 * t5075 + 0.33163888888888888888e-2 * t5078 + 0.33163888888888888888e-2 * t5080 + 0.24872916666666666666e-2 * t5178 - 0.33163888888888888888e-2 * t5189 + 0.22109259259259259258e-2 * t5197 - 0.33163888888888888888e-2 * t5201 - 0.55273148148148148147e-3 * t5206 + 0.193e0 * t1693 * t4827;
    (t5193, t5196, t5197, t5199, t5200, t5201, t5203, t5204, t5205, t5206, t5210)
}
