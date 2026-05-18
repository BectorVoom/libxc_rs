//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1261/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1261<F: Float>(t237: F, t6282: F, t2312: F, t889: F, t8177: F, t2318: F, t1209: F, t2204: F, t3046: F, t6149: F, t6165: F, t1174: F, t18480: F, t6150: F) -> (F, F, F, F, F, F) {
    let t22180 = t237 * t6282;
    let t22181 = t889 * t2312;
    let t22184 = F::new(0.30762056574649219974e4) * t22180 * t8177 * t22181;
    let t22185 = t237 * t2318;
    let t22188 = F::new(0.10526802520742363173e2) * t22185 * t1209 * t22181;
    let t22190 = t6149 * t3046 * t2204;
    let t22193 = t6165 * t3046 * t2204;
    let t22196 = t18480 * t1174 * t6150;
    (t22181, t22184, t22188, t22190, t22193, t22196)
}
