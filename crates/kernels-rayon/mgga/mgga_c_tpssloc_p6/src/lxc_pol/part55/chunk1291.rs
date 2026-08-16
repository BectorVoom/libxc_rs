//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1291/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1291(t32578: f64, t45844: f64, t12571: f64, t117727: f64, t117757: f64, t117762: f64, t119880: f64, t119884: f64, t119955: f64, t119971: f64, t119975: f64, t119990: f64, t1410: f64, t27363: f64, t31006: f64, t31024: f64, t31857: f64, t31864: f64, t31868: f64, t32579: f64, t32587: f64, t32590: f64, t33107: f64, t33119: f64, t33669: f64, t34222: f64, t7254: f64, t8307: f64, t8308: f64, t8513: f64, t8663: f64, t8856: f64) -> f64 {
    let t125837 = t45844 * t32578;
    let t125842 = t12571 * t32578;
    let t125855 = -5.0_f64 / 24.0_f64 * t32579 * t119990 + 5.0_f64 / 72.0_f64 * t31857 * t34222 + 5.0_f64 / 72.0_f64 * t31868 * t34222 + 5.0_f64 / 72.0_f64 * t8663 * t8513 * t8307 * t27363 + 5.0_f64 / 72.0_f64 * t117762 * t33119 + 5.0_f64 / 72.0_f64 * t32590 * t119971 + 5.0_f64 / 72.0_f64 * t32590 * t119975 + 5.0_f64 / 144.0_f64 * t119955 * t8856 - 5.0_f64 / 24.0_f64 * t125837 * t31006 + 5.0_f64 / 72.0_f64 * t33669 * t32587 + 5.0_f64 / 72.0_f64 * t125842 * t31024 - 5.0_f64 / 24.0_f64 * t117757 * t33107 + 5.0_f64 / 6.0_f64 * t117727 * t119880 + 5.0_f64 / 6.0_f64 * t117727 * t119884 - 5.0_f64 / 18.0_f64 * t31864 * t8308 * t1410 * t7254;
    t125855
}
