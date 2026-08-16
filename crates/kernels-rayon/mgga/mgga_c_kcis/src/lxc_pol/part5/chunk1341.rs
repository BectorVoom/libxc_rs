//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1341/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1341(t518: f64, t6957: f64, t1419: f64, t5457: f64, t5458: f64, t5481: f64, t1098: f64, t7242: f64, t3814: f64, t531: f64, t21641: f64, t16373: f64, t21625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22164 = t518 * t6957;
    let t22165 = t22164 * t1419;
    let t22166 = t5457 * t22165;
    let t22169 = t5458 * t5481;
    let t22170 = t5457 * t22169;
    let t22175 = t1098 * t7242;
    let t22177 = t3814 * t531;
    let t22178 = t22177 * t21641;
    let t22181 = t16373 * t21625;
    (t22165, t22166, t22169, t22170, t22175, t22178, t22181)
}
