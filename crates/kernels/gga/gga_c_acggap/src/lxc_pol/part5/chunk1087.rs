//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1087/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1087<F: Float>(t13298: F, t13299: F, t22040: F, t525: F, t1089: F, t1180: F, t13399: F, t13400: F, t16946: F, t16950: F, t22021: F, t22023: F, t22032: F, t22038: F, t3396: F, t418: F, t4680: F, t5111: F, t535: F, t5795: F, t5931: F) -> (F,) {
    let t22043 = t13298 * t13299 * t525 * t22040;
    let t22046 = -0.17149607247227894789e-2 * t1180 * t4680 * t5795 + 0.13719685797782315831e-1 * t3396 * t4680 * t5931 + 0.68598428988911579156e-2 * t22021 + 0.17149607247227894789e-2 * t22023 - 0.68598428988911579156e-2 * t418 * t1089 * t535 * t5111 - 0.42874018118069736972e-2 * t22032 - 0.17149607247227894789e-1 * t16946 - 0.85748036236139473945e-2 * t16950 - 0.68598428988911579156e-2 * t22038 + 0.68598428988911579156e-2 * t22043 + t13399 + 0.25724410870841842183e-2 * t13400;
    (t22046,)
}
