//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 782/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk782<F: Float>(t12141: F, t12241: F, t184: F, t21: F, t363: F, t3659: F, t3658: F, t648: F, t3664: F, t1078: F, t2304: F, t3539: F, t604: F) -> (F, F, F, F, F) {
    let t12242 = t12141 + t12241;
    let t12243 = t12242 * t184;
    let t12244 = t12243 * t21;
    let t12253 = t3659 * t363;
    let t12256 = t3658 * t648;
    let t12257 = t12256 * t3664;
    let t12260 = t1078 * t2304;
    let t12261 = t12260 * t3664;
    let t12277 = t3539 * t604;
    (t12244, t12253, t12257, t12261, t12277)
}
