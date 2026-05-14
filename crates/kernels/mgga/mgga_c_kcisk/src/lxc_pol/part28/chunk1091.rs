//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1091/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1091<F: Float>(t2023: F, t2642: F, t2364: F, t18338: F, t1060: F, t6763: F, t18327: F, t6758: F, t18445: F, t4998: F, t9213: F, t2013: F, t2020: F, t9226: F, t1636: F, t1775: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24991 = t2642 * t2023;
    let t24992 = t2364 * t24991;
    let t24993 = t18338 * t24992;
    let t24996 = t2642 * t1060;
    let t24997 = t6763 * t24996;
    let t24998 = t18327 * t24997;
    let t25001 = t6758 * t24996;
    let t25002 = t18445 * t25001;
    let t25006 = t4998 * t9213;
    let t25007 = t2013 * t25006;
    let t25009 = t2020 * t9226;
    let t25010 = t25009 * t1636;
    let t25011 = t1775 * t25010;
    (t24991, t24992, t24993, t24997, t24998, t25001, t25002, t25007, t25011)
}
