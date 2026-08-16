//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1300/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1300(t1165: f64, t22040: f64, t4267: f64, t4282: f64, t5551: f64, t997: f64, t14357: f64, t14376: f64, t175: f64, t18719: f64, t21693: f64, t24145: f64, t24147: f64, t24149: f64, t24151: f64, t24153: f64, t24155: f64, t418: f64, t4352: f64) -> f64 {
    let t24165 = t4282 * t1165 * t4267 * t22040;
    let t24168 = t997 * t5551;
    let t24170 = -0.34299214494455789578e-2_f64 * t24145 - 0.22675591804667994221e-1_f64 * t24147 - 0.12004725073059526353e-1_f64 * t24149 - 7.0_f64 / 36.0_f64 * t24151 - 7.0_f64 / 36.0_f64 * t24153 - 7.0_f64 / 24.0_f64 * t24155 - 0.25724410870841842183e-1_f64 * t418 * t4352 * t175 * t21693 - 7.0_f64 / 36.0_f64 * t18719 - 0.17149607247227894789e-2_f64 * t14357 + 0.34299214494455789578e-1_f64 * t24165 + 0.34299214494455789578e-2_f64 * t14376 - 0.32012600194825403606e-1_f64 * t24168;
    t24170
}
