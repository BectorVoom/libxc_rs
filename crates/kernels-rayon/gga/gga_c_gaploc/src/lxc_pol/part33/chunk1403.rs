//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1403/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1403(t12079: f64, t31299: f64, t31346: f64, t35054: f64, t35075: f64, t35090: f64, t35094: f64, t35097: f64, t35100: f64, t35104: f64, t35110: f64, t35113: f64, t35116: f64, t35120: f64, t38731: f64, t4372: f64, t6710: f64, t6711: f64, t6716: f64, t6717: f64) -> f64 {
    let t38824 = t35054 - 0.23005755572352449806e2_f64 * t6710 * t6711 * t38731 + 0.13803453343411469884e2_f64 * t6716 * t6717 * t38731 + t31299 - t35075 + 0.92686455430723328401e-1_f64 * t12079 * t4372 - t35090 - t35094 - t35097 + t35100 - t35104 + t35110 + t35113 + t35116 - t35120 - 0.76685851907841499354e0_f64 * t31346;
    t38824
}
