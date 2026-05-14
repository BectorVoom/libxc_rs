//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 553/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk553<F: Float>(t1339: F, t8181: F, t1220: F, t5610: F, t8064: F, t8075: F, t8080: F, t8084: F, t8087: F, t8091: F, t8095: F, t8165: F, t8173: F, t8178: F, t8071: F, t504: F) -> (F, F, F) {
    let t8182 = t1339 * t8181;
    let t8184 = 0.49745833333333333332e-2 * t8075 - 0.33163888888888888888e-2 * t8080 - 0.55273148148148148147e-3 * t8084 + 0.33163888888888888888e-2 * t8087 + 0.16581944444444444444e-2 * t8091 + 0.27636574074074074073e-2 * t8095 + 0.24872916666666666666e-2 * t8165 + 0.22109259259259259258e-2 * t5610 + 0.193e0 * t1220 * t8064 - 0.33163888888888888888e-2 * t8173 + 0.22109259259259259258e-2 * t8178 - 0.33163888888888888888e-2 * t8182;
    let t8185 = t8071 + t8184;
    let t8186 = t8185 * t504;
    (t8182, t8185, t8186)
}
