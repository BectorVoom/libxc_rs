//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1300/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1300<F: Float>(t1165: F, t22040: F, t4267: F, t4282: F, t5551: F, t997: F, t14357: F, t14376: F, t175: F, t18719: F, t21693: F, t24145: F, t24147: F, t24149: F, t24151: F, t24153: F, t24155: F, t418: F, t4352: F) -> F {
    let t24165 = t4282 * t1165 * t4267 * t22040;
    let t24168 = t997 * t5551;
    let t24170 = -F::new(0.34299214494455789578e-2) * t24145 - F::new(0.22675591804667994221e-1) * t24147 - F::new(0.12004725073059526353e-1) * t24149 - F::new(7.0) / F::new(36.0) * t24151 - F::new(7.0) / F::new(36.0) * t24153 - F::new(7.0) / F::new(24.0) * t24155 - F::new(0.25724410870841842183e-1) * t418 * t4352 * t175 * t21693 - F::new(7.0) / F::new(36.0) * t18719 - F::new(0.17149607247227894789e-2) * t14357 + F::new(0.34299214494455789578e-1) * t24165 + F::new(0.34299214494455789578e-2) * t14376 - F::new(0.32012600194825403606e-1) * t24168;
    t24170
}
