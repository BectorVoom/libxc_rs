//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 810/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk810<F: Float>(t1993: F, t9155: F, t1676: F, t8584: F, t4790: F, t8607: F, t5400: F, t9124: F, t1965: F, t9103: F, t240: F, t7218: F, t7580: F) -> (F, F, F, F, F, F, F) {
    let t24608 = t9155 * t1993;
    let t24727 = t8584 * t1676;
    let t24747 = t8607 * t4790;
    let t24774 = t9124 * t5400;
    let t24785 = t9103 * t1965;
    let t24819 = t240 * t8584;
    let t24876 = t7580 * t7218;
    (t24608, t24727, t24747, t24774, t24785, t24819, t24876)
}
