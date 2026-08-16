//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1099/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1099(t35691: f64, t35705: f64, t37815: f64, t37816: f64, t37818: f64, t40343: f64, t43390: f64, t43391: f64, t43392: f64, t43393: f64, t43422: f64, t46992: f64, t46995: f64, t46999: f64, t47004: f64, t47006: f64, t47008: f64, t47011: f64) -> f64 {
    let t48849 = -0.1702583995731913576e-4_f64 * t46992 + 0.20496175532535769483e-3_f64 * t35691 - 0.1702583995731913576e-4_f64 * t46995 + 0.59620292925746722033e-2_f64 * t40343 - 0.19863479950205658386e-4_f64 * t46999 + t43390 - t43391 + t43392 + t43393 - 0.39726959900411316773e-4_f64 * t47004 + 0.19863479950205658386e-4_f64 * t47006 - 0.79828278012425390427e-1_f64 * t47008 - t37815 - t37816 - t37818 - 0.70441376091769752081e-2_f64 * t35705 + 0.1702583995731913576e-4_f64 * t47011 - t43422;
    t48849
}
