//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1260/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1260<F: Float>(t112520: F, t9664: F, t10473: F, t9681: F, t32916: F, t4998: F, t112395: F, t9649: F, t17182: F, t33022: F, t1333: F, t32927: F, t32942: F, t32955: F, t32889: F, t9657: F) -> (F, F, F, F, F, F, F, F) {
    let t112521 = t9664 * t112520;
    let t112523 = t10473 * t9681;
    let t112530 = t9664 * t4998 * t32916;
    let t112534 = t9649 * t112395;
    let t112539 = t9664 * t17182 * t33022;
    let t112541 = t1333 * t32927;
    let t112547 = t32942 * t32955;
    let t112549 = t9657 * t32889;
    (t112521, t112523, t112530, t112534, t112539, t112541, t112547, t112549)
}
