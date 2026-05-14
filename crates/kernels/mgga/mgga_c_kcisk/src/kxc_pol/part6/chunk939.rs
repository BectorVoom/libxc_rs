//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 939/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk939<F: Float>(t2326: F, t7706: F, t14978: F, t1312: F, t14909: F, t3952: F, t30852: F, t4391: F, t7710: F, t4400: F, t2059: F, t8398: F, t30557: F, t30561: F, t30564: F, t30567: F, t30641: F, t30644: F, t30660: F, t30662: F, t30664: F, t30668: F, t6568: F, t7804: F) -> (F, F, F, F, F, F) {
    let t31464 = t7706 * t2326;
    let t31465 = t14978 * t31464;
    let t31466 = t1312 * t31465;
    let t31469 = t14909 * t31464;
    let t31470 = t3952 * t31469;
    let t31473 = t4391 * t30852;
    let t31474 = t3952 * t31473;
    let t31477 = t7710 * t2326;
    let t31478 = t4400 * t31477;
    let t31479 = t1312 * t31478;
    let t31483 = t4400 * t2059 * t8398;
    let t31484 = t1312 * t31483;
    let t31492 = 0.35089340384731224426e1 * t6568 * t7804 - t30557 + t30561 - t30564 + t30567 + t30641 + t30644 + t30660 + t30662 + t30664 - t30668;
    (t31466, t31470, t31474, t31479, t31484, t31492)
}
