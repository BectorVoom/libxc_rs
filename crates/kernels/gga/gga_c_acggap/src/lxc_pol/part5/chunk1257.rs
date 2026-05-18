//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1257/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1257<F: Float>(t5975: F, t997: F, t6205: F, t6211: F, t1016: F, t1083: F, t1181: F, t13861: F, t1460: F, t1487: F, t1524: F, t17729: F, t17733: F, t17740: F, t3201: F, t398: F, t418: F, t4735: F, t4762: F, t4838: F, t4875: F, t506: F, t513: F, t5819: F, t6337: F) -> F {
    let t23154 = t997 * t5975;
    let t23156 = t997 * t6205;
    let t23158 = t997 * t6211;
    let t23172 = -F::new(0.34299214494455789578e-2) * t418 * t398 * t3201 * t5819 + F::new(0.34299214494455789578e-2) * t17729 - F::new(0.17149607247227894789e-2) * t418 * t398 * t1083 * t4875 * t513 - F::new(0.34299214494455789578e-2) * t418 * t398 * t1083 * t1487 * t1524 - F::new(0.17149607247227894789e-2) * t418 * t398 * t1083 * t506 * t4838 - F::new(0.40015750243531754508e-1) * t23154 + F::new(0.16006300097412701803e-1) * t23156 - F::new(0.16006300097412701803e-1) * t23158 + F::new(0.10289764348336736873e-1) * t4735 * t1181 * t1016 * t1487 * t1460 + F::new(0.12004725073059526352e-1) * t13861 + F::new(0.10289764348336736873e-1) * t4735 * t1181 * t6337 * t4762 - F::new(0.64025200389650807212e-1) * t17733 + F::new(0.34299214494455789578e-2) * t17740;
    t23172
}
