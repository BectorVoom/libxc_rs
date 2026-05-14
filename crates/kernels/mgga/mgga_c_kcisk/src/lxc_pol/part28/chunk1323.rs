//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1323/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1323<F: Float>(t116477: F, t9649: F, t17182: F, t34136: F, t9664: F, t34181: F, t10487: F, t1791: F, t10798: F, t33031: F, t34022: F, t1871: F, t6944: F, t32909: F, t34125: F, t17353: F, t34012: F) -> (F, F, F, F, F, F, F, F, F) {
    let t116601 = t9649 * t116477;
    let t116620 = 0.13888888888888888889e-1 * t9664 * t17182 * t34136;
    let t116621 = t17182 * t34181;
    let t116623 = 0.69444444444444444446e-2 * t9664 * t116621;
    let t116625 = 0.26805555555555555556e-2 * t9649 * t116621;
    let t116645 = t1791 * t10487;
    let t116651 = t33031 * t10798 * t34022;
    let t116664 = t6944 * t1871;
    let t116672 = 0.18518518518518518519e-1 * t34125 * t32909;
    let t116676 = t17353 * t34012;
    (t116601, t116620, t116623, t116625, t116645, t116651, t116664, t116672, t116676)
}
