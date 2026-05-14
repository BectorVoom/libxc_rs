//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1208/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1208<F: Float>(t164: F, t657: F, t11985: F, t2618: F, t642: F, t7069: F, t5217: F, t7291: F, t18837: F, t2041: F, t2464: F, t5032: F, t1785: F, t7268: F, t5038: F, t11196: F, t2399: F) -> (F, F, F, F, F, F, F, F, F) {
    let t62249 = t164 * t657;
    let t62760 = t2618 * t11985;
    let t62789 = t642 * t7069;
    let t63008 = t7291 * t5217;
    let t63011 = t18837 * t2041;
    let t63573 = t2464 * t5032;
    let t63617 = t7268 * t1785;
    let t64506 = t2464 * t5038;
    let t64905 = t2399 * t11196;
    (t62249, t62760, t62789, t63008, t63011, t63573, t63617, t64506, t64905)
}
