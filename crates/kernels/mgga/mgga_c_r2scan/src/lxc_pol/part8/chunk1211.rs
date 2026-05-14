//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1211/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1211<F: Float>(t1591: F, t7983: F, t538: F, t910: F, t19890: F, t2147: F, t7624: F, t22767: F, t8124: F, t13866: F, t2195: F, t2183: F, t19790: F, t921: F, t19789: F, t22947: F) -> (F, F, F, F, F, F, F) {
    let t25243 = t1591 * t7983;
    let t25299 = t538 * t910;
    let t25322 = t2147 * t19890 * t7624;
    let t25323 = 0.2037639021386884617e0 * t25322;
    let t25347 = t22767 * t8124;
    let t25359 = t2195 * t13866;
    let t25363 = t2183 * t13866;
    let t25397 = t19790 * t921;
    let t25399 = t22947 * t19789 * t25397;
    (t25243, t25299, t25323, t25347, t25359, t25363, t25399)
}
