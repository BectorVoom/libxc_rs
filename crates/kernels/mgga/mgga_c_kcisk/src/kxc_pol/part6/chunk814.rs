//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 814/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk814<F: Float>(t1248: F, t1720: F, t28389: F, t28373: F, t4893: F, t28381: F, t1714: F, t29138: F, t7115: F, t8708: F, t7099: F, t23460: F, t23606: F, t23609: F, t29082: F, t29085: F, t29091: F, t29097: F, t29152: F) -> (F, F, F, F, F, F, F) {
    let t29155 = t1248 * t1720 * t28389;
    let t29161 = t1248 * t4893 * t28373;
    let t29164 = t1248 * t1720 * t28381;
    let t29166 = t1714 * t29138;
    let t29168 = t7115 * t8708;
    let t29170 = t7099 * t8708;
    let t29172 = -0.33547222222222222222e0 * t29082 + 0.12077e1 * t29085 - 0.181155e1 * t29091 - 0.301925e0 * t29097 - 0.73586666666666666666e-1 * t29152 - 0.16557e0 * t29155 + 0.20128333333333333333e0 * t23460 + 0.11038e0 * t23606 + 0.33114e0 * t23609 + 0.33114e0 * t29161 - 0.99342e0 * t29164 + 0.16504875e0 * t29166 + 0.247573125e0 * t29168 - 0.3883875e1 * t29170;
    (t29155, t29161, t29164, t29166, t29168, t29170, t29172)
}
