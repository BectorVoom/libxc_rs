//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 892/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk892<F: Float>(t11443: F, t28414: F, t706: F, t11417: F, t11418: F, t28368: F, t2488: F, t8536: F, t7055: F, t1876: F, t4598: F, t11328: F, t4595: F) -> (F, F, F, F, F, F, F) {
    let t28885 = t11443 * t28414;
    let t28886 = t706 * t28885;
    let t28894 = t11417 * t11418 * t28368;
    let t28897 = t2488 * t8536;
    let t28898 = t7055 * t28897;
    let t28902 = t1876 * t4598 * t28368;
    let t28906 = t4595 * t11328 * t28368;
    (t28885, t28886, t28894, t28897, t28898, t28902, t28906)
}
