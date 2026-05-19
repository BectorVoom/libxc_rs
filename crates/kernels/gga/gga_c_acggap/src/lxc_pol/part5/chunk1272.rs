//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1272/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1272<F: Float>(t1111: F, t1165: F, t20545: F, t3391: F, t1090: F, t1150: F, t1181: F, t12473: F, t1713: F, t1782: F, t18097: F, t18103: F, t18105: F, t18107: F, t18109: F, t18111: F, t18119: F, t1889: F, t336: F, t3565: F, t367: F, t4417: F, t4463: F, t4735: F, t4757: F) -> F {
    let t23511 = t3391 * t1165 * t20545 * t1111;
    let t23529 = F::new(35.0) / F::new(54.0) * t18097 + F::cast_from(0.10289764348336736874e-1_f64) * t4735 * t1181 * t4417 * t4757 + F::cast_from(0.16006300097412701803e-1_f64) * t18103 + F::cast_from(0.80031500487063509016e-2_f64) * t18105 + F::cast_from(0.10289764348336736874e-1_f64) * t23511 + F::cast_from(0.80031500487063509016e-2_f64) * t18107 + F::cast_from(0.40015750243531754508e-2_f64) * t18109 + F::cast_from(0.68598428988911579156e-2_f64) * t18111 + F::cast_from(0.17149607247227894789e-1_f64) * t4463 * t1181 * t1889 * t1090 + F::cast_from(0.68598428988911579156e-2_f64) * t18119 + t367 * t336 * t12473 * t1782 / F::new(48.0) + t1150 * t336 * t3565 * t1713 / F::new(16.0);
    t23529
}
