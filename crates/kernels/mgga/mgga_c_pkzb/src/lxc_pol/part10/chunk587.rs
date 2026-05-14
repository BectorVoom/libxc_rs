//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 587/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk587<F: Float>(t2328: F, t900: F, t2295: F, t2297: F, t890: F, t898: F, t2312: F, t881: F, t2317: F, t2320: F, t154: F, t386: F, t486: F, t385: F, t405: F, t67: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2330 = 0.11696447245269292414e1 * t2328 * t900;
    let t2332 = t2295 * t2297 * t890;
    let t2334 = 0.11696447245269292414e1 * t898 * t2332;
    let t2336 = t881 * t2312 * t890;
    let t2338 = 0.5848223622634646207e0 * t898 * t2336;
    let t2339 = t2317 * t2297;
    let t2340 = t2339 * t2320;
    let t2342 = 0.17315859105681463759e2 * t898 * t2340;
    let t2344 = t154 * t486 * t386;
    let t2346 = t385 * t2344 / 432.0;
    let t2347 = t67 * t405;
    (t2330, t2332, t2334, t2336, t2338, t2340, t2342, t2344, t2346, t2347)
}
