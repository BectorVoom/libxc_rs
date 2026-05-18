//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1050/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1050<F: Float>(t3241: F, t3244: F, t1058: F, t3197: F, t11132: F, t3163: F, t3172: F, t3161: F, t126: F, t373: F, t828: F) -> (F, F, F, F, F, F) {
    let t11886 = t3241 * t3244;
    let t11888 = t3197 * t1058;
    let t11890 = F::new(0.25925925925925925926e-1) * t11132;
    let t11916 = t3172 * t3163;
    let t11917 = t3161 * t11916;
    let t11921 = t126 * t373;
    let t11922 = t828 * t11921;
    (t11886, t11888, t11890, t11917, t11921, t11922)
}
