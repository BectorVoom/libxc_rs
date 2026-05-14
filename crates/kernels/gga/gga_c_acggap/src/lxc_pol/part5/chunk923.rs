//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 923/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk923<F: Float>(t13292: F, t1530: F, t13285: F, t13298: F, t176: F, t5284: F, t8401: F, t3409: F, t4443: F, t1511: F, t3573: F, t1089: F, t175: F, t322: F, t384: F, t4099: F) -> (F, F, F, F, F, F) {
    let t17179 = t1530 * t13292;
    let t17185 = t1530 * t13285;
    let t17198 = t13298 * t176 * t8401 * t5284;
    let t17205 = t3409 * t4443;
    let t17216 = t3573 * t1511;
    let t17221 = t384 * t1089 * t175 * t4099 * t322;
    (t17179, t17185, t17198, t17205, t17216, t17221)
}
