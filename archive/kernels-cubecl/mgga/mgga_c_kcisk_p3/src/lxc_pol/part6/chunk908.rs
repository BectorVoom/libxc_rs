//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 908/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk908<F: Float>(t11003: F, t1248: F, t28369: F, t1720: F, t28389: F, t28373: F, t4893: F, t28381: F, t1714: F, t29138: F, t7115: F, t8708: F) -> (F, F, F, F, F, F) {
    let t29152 = t1248 * t11003 * t28369;
    let t29155 = t1248 * t1720 * t28389;
    let t29161 = t1248 * t4893 * t28373;
    let t29164 = t1248 * t1720 * t28381;
    let t29166 = t1714 * t29138;
    let t29168 = t7115 * t8708;
    (t29152, t29155, t29161, t29164, t29166, t29168)
}
