//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 901/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk901(t9731: f64, t9734: f64, t9742: f64, t9745: f64, t9748: f64, t9751: f64, t9754: f64, t9758: f64, t9761: f64, t9764: f64, t9766: f64, t9768: f64, t9771: f64) -> f64 {
    let t10932 = 0.34752370105806885418e-3_f64 * t9731 - 0.38647271295071362317e-7_f64 * t9734 + 0.43047021936487268522e-6_f64 * t9742 + 0.17376185052903442709e-3_f64 * t9745 - 0.13900948042322754167e-3_f64 * t9748 - 0.13900948042322754167e-3_f64 * t9751 + 0.41702844126968262501e-3_f64 * t9754 + 0.10005428175813516294e-8_f64 * t9758 + 0.15458908518028544927e-5_f64 * t9761 - 0.51491428373437201896e-5_f64 * t9764 - 0.34752370105806885418e-3_f64 * t9766 + 0.28960308421505737848e-5_f64 * t9768 - 0.45018799441230669486e-7_f64 * t9771;
    t10932
}
