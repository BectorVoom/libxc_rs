//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 192/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk192<F: Float>(t51: F, t600: F, t164: F, t592: F, t499: F, t66: F, t168: F, t167: F, t180: F, t589: F) -> (F, F, F, F, F) {
    let t601 = t51 * t600;
    let t603 = t592 * t601 * t164;
    let t607 = F::cast_from(1.0_f64) / t66 / t499;
    let t608 = t168 * t607;
    let t611 = F::cast_from(0.10003937560882938627e-2_f64) * t167 * t608 * t180;
    let t612 = t167 * t589;
    (t603, t607, t608, t611, t612)
}
