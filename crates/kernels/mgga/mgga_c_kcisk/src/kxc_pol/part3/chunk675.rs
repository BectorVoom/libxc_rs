//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 675/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk675<F: Float>(t10487: F, t708: F, t10441: F, t4595: F, t1648: F, t4652: F, t7028: F, t1417: F, t4686: F, t4626: F, t4654: F, t1889: F, t3517: F, t10660: F, t1882: F, t706: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11328 = t708 * t10487;
    let t11330 = t4595 * t11328 * t10441;
    let t11334 = t708 * t1648 * t4652;
    let t11335 = t7028 * t11334;
    let t11338 = t1417 * t4686;
    let t11340 = t1417 * t4626;
    let t11342 = t1417 * t4654;
    let t11344 = t3517 * t1889;
    let t11346 = t1882 * t10660;
    let t11347 = t706 * t11346;
    (t11330, t11334, t11335, t11338, t11340, t11342, t11344, t11346, t11347)
}
