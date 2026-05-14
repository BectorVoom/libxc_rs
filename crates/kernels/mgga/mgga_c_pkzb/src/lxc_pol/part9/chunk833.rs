//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 833/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk833<F: Float>(t2320: F, t6122: F, t2234: F, t853: F, t2197: F, t2242: F, t851: F, t2240: F, t2312: F, t891: F, t889: F, t2273: F, t872: F, t2281: F, t870: F, t6087: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6324 = t6122 * t2320;
    let t6327 = t853 * t2234;
    let t6329 = 6.0 * t2197 * t6327;
    let t6331 = t2234 * t2242 * t851;
    let t6333 = 0.48245938496077605201e2 * t2240 * t6331;
    let t6334 = t891 * t2312;
    let t6337 = t2312 * t2320;
    let t6338 = t6337 * t889;
    let t6341 = t872 * t2273;
    let t6345 = t2273 * t2281 * t870;
    let t6348 = 0.53272592592592592592e-1 * t6087;
    (t6324, t6327, t6329, t6331, t6333, t6334, t6337, t6338, t6341, t6345, t6348)
}
