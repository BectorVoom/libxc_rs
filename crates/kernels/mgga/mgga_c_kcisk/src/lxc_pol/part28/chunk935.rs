//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 935/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk935<F: Float>(t1871: F, t8664: F, t1895: F, t1869: F, t9061: F, t1900: F, t6697: F, t6714: F, t1800: F, t6713: F, t4581: F, t8870: F, t1799: F, t1333: F, t8859: F, t2364: F, t6702: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22249 = t8664 * t1871;
    let t22250 = t22249 * sigma2;
    let t22251 = t22250 * t1895;
    let t22252 = t1869 * t22251;
    let t22254 = t9061 * sigma2;
    let t22255 = t22254 * t1900;
    let t22256 = t1869 * t22255;
    let t22258 = t6697 * t6714;
    let t22259 = t1800 * t22258;
    let t22260 = t6713 * t22259;
    let t22262 = t4581 * t8870;
    let t22263 = t1799 * t22262;
    let t22265 = t1333 * t8859;
    let t22267 = t2364 * t6702;
    (t22249, t22250, t22252, t22254, t22256, t22260, t22263, t22265, t22267)
}
