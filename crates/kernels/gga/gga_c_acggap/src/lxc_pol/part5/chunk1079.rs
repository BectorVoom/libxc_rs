//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1079/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1079<F: Float>(t1165: F, t12991: F, t4267: F, t5099: F, t13087: F, t6090: F, t4389: F, t5743: F, t1180: F, t13274: F, t13276: F, t13278: F, t13280: F, t13282: F, t16765: F, t16769: F, t16779: F, t16781: F, t1879: F, t955: F) -> (F,) {
    let t21832 = t12991 * t1165 * t4267 * t5099;
    let t21834 = t13087 * t6090;
    let t21844 = t4389 * t5743;
    let t21847 = -0.12862205435420921092e-2 * t1180 * t1165 * t1879 * t955 + 0.68598428988911579156e-2 * t21832 + 0.32012600194825403606e-1 * t21834 - 0.13719685797782315831e-1 * t16765 - 0.68598428988911579156e-2 * t16769 + 455.0 / 648.0 * t13274 - 35.0 / 108.0 * t13276 - 35.0 / 216.0 * t13278 - 35.0 / 216.0 * t13280 - 35.0 / 432.0 * t13282 - 0.17149607247227894789e-2 * t16779 + 0.16006300097412701803e-1 * t21844 - 0.34299214494455789577e-2 * t16781;
    (t21847,)
}
