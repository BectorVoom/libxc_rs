//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 921/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk921<F: Float>(t18217: F, t242: F, t3972: F, t3977: F, t1175: F, t3821: F, t729: F, t1131: F, t4005: F, t1168: F, t13830: F, t5181: F, t713: F) -> (F, F, F, F, F, F, F, F) {
    let t18218 = t242 * t18217;
    let t18221 = t3977 * t3972;
    let t18222 = t242 * t18221;
    let t18226 = t729 * t1175 * t3821;
    let t18230 = t729 * t4005 * t1131;
    let t18233 = t13830 * t1168;
    let t18234 = t242 * t18233;
    let t18238 = t729 * t5181 * t713;
    (t18218, t18221, t18222, t18226, t18230, t18233, t18234, t18238)
}
