//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1089/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1089<F: Float>(t1017: F, t1165: F, t1173: F, t1180: F, t1181: F, t1426: F, t16980: F, t16988: F, t16990: F, t1743: F, t1748: F, t1772: F, t22048: F, t22068: F, t22080: F, t22082: F, t22085: F, t368: F, t397: F, t398: F, t418: F, t4289: F, t4298: F, t4313: F, t530: F, t5616: F, t6394: F, t922: F, t955: F) -> (F,) {
    let t22091 = 0.34299214494455789578e-2 * t1173 * t1181 * t530 * t22048 + 0.34299214494455789578e-2 * t1180 * t1181 * t4298 * t6394 - 0.17149607247227894789e-2 * t1180 * t1181 * t4289 * t5616 + 0.10289764348336736873e-1 * t1173 * t1165 * t4313 * t1748 * t1017 + 0.42874018118069736972e-3 * t22068 - 0.17149607247227894789e-2 * t16980 - 0.25724410870841842183e-2 * t16988 + 0.85748036236139473945e-2 * t418 * t1426 * t368 * t1772 * t922 + 0.85748036236139473944e-3 * t22080 + 0.40015750243531754508e-2 * t22082 + 0.48018900292238105408e-1 * t16990 - 0.42874018118069736972e-3 * t22085 - 0.42874018118069736972e-3 * t397 * t398 * t1743 * t955;
    (t22091,)
}
