//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1095/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1095<F: Float>(t5708: F, t5713: F, t2011: F, t5939: F, t757: F, t2026: F, t2032: F, t2038: F, t2040: F, t1478: F, t301: F, t154: F, t276: F, t655: F) -> (F, F, F, F, F, F) {
    let t18028 = t5713 * t5708;
    let t18033 = t757 * t5939 * t2011;
    let t18036 = t2026 * t5939 * t2032;
    let t18039 = t2038 * t5939 * t2040;
    let t18060 = t1478 * t301;
    let t18063 = t276 * t154 * t18060 * t655;
    (t18028, t18033, t18036, t18039, t18060, t18063)
}
