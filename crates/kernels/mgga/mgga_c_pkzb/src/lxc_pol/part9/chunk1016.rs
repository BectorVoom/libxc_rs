//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1016/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1016<F: Float>(t2038: F, t2040: F, t5939: F, t1478: F, t301: F, t154: F, t276: F, t655: F, t1843: F, t5688: F, t2048: F, t5537: F, t2050: F, t2057: F, t5665: F, t735: F) -> (F, F, F, F, F, F, F) {
    let t18039 = t2038 * t5939 * t2040;
    let t18060 = t1478 * t301;
    let t18063 = t276 * t154 * t18060 * t655;
    let t18067 = t276 * t154 * t5688 * t1843;
    let t18071 = t276 * t154 * t2048 * t5537;
    let t18073 = t2057 * t2050;
    let t18079 = t735 * t5665;
    (t18039, t18060, t18063, t18067, t18071, t18073, t18079)
}
