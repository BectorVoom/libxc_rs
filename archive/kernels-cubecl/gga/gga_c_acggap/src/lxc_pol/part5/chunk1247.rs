//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1247/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1247<F: Float>(t6220: F, t997: F, t6228: F, t6200: F, t1140: F, t5913: F, t1017: F, t1089: F, t1095: F, t12576: F, t1323: F, t1327: F, t175: F, t17530: F, t1795: F, t18129: F, t20764: F, t21673: F, t335: F, t367: F, t418: F, t4352: F, t4495: F, t4593: F, t5183: F, t6374: F, t839: F, t922: F, t960: F) -> F {
    let t22890 = t997 * t6220;
    let t22892 = t997 * t6228;
    let t22894 = t997 * t6200;
    let t22906 = t1140 * t5913;
    let t22928 = -F::cast_from(0.16006300097412701803e-1_f64) * t22890 + F::cast_from(0.80031500487063509015e-2_f64) * t22892 - F::cast_from(0.12004725073059526352e-1_f64) * t22894 + F::cast_from(0.17149607247227894789e-2_f64) * t418 * t1089 * t1095 * t1795 * t839 - F::cast_from(0.25724410870841842183e-1_f64) * t418 * t4352 * t175 * t21673 - F::cast_from(0.12004725073059526352e-1_f64) * t17530 - F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t22906 + t335 * t18129 * t1323 / F::cast_from(12.0_f64) + t335 * t4593 * t5183 / F::cast_from(12.0_f64) + t335 * t4593 * t4495 / F::cast_from(24.0_f64) + t367 * t18129 * t1327 / F::cast_from(12.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4.0_f64) * t12576 * t960 * t6374 * t922 - t367 * t960 * t20764 * t1017 / F::cast_from(16.0_f64);
    t22928
}
