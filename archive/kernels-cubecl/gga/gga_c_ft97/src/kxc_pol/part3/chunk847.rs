//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 847/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk847<F: Float>(t4714: F, t574: F, t616: F, t167: F, t16919: F, t1053: F, t3565: F, t2179: F, t144: F, t4823: F, t9419: F, t3408: F, t920: F) -> (F, F, F, F, F, F) {
    let t17174 = t574 * t616 * t4714;
    let t17178 = t574 * t167 * t16919;
    let t17181 = t1053 * t3565;
    let t17182 = t2179 * t17181;
    let t17183 = t144 * t17182;
    let t17186 = t9419 * t4823;
    let t17189 = t920 * t3408;
    (t17174, t17178, t17182, t17183, t17186, t17189)
}
