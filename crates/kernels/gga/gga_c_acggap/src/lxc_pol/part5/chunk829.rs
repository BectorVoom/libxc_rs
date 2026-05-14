//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 829/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk829<F: Float>(t1163: F, t1165: F, t13232: F, t3176: F, t3171: F, t3375: F, t1159: F, t3242: F, t1162: F, t1167: F, t1172: F, t12726: F, t1177: F, t839: F, t944: F, t3206: F, t366: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13235 = t1163 * t1165 * t13232 * t3176;
    let t13253 = t3375 * t3171;
    let t13259 = t3242 * t1159;
    let t13260 = t13259 * t1162;
    let t13261 = t13260 * t1167;
    let t13263 = t12726 * t1172;
    let t13264 = t13263 * t1177;
    let t13268 = t944 * t839;
    let t13273 = t3206 * t366;
    (t13235, t13253, t13259, t13260, t13261, t13263, t13264, t13268, t13273)
}
