//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 851/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk851<F: Float>(t2242: F, t6143: F, t6142: F, t339: F, t346: F, t2204: F, t836: F, t2203: F, t2209: F, t6087: F, t6090: F, t6093: F, t6108: F) -> (F, F, F, F, F, F, F, F) {
    let t6144 = t6143 * t2242;
    let t6146 = F::cast_from(0.96491876992155210402e2_f64) * t6142 * t6144;
    let t6149 = F::cast_from(1.0_f64) / t339 / t346 / F::cast_from(4.0_f64);
    let t6150 = t2204 * t836;
    let t6151 = t6149 * t6150;
    let t6153 = t2203 * t836;
    let t6154 = t6153 * t2209;
    let t6156 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t6087;
    let t6158 = -t6156 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6090 - t6093 + t6108;
    (t6144, t6146, t6149, t6150, t6151, t6154, t6156, t6158)
}
