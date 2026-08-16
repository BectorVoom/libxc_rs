//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2379/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2379<F: Float>(t2694: F, t9784: F, t16: F, t2236: F, t240: F, t236: F, t243: F, t281: F, t39644: F, t10871: F, t775: F, t10696: F, t72: F) -> (F, F, F, F, F, F) {
    let t40639 = t9784 * t2694;
    let t40648 = t2236 * t16;
    let t40649 = F::cast_from(1.0_f64) / t40648;
    let t40650 = t40649 * t240;
    let t40654 = F::cast_from(0.47607864835161149081e-7_f64) * t39644 * t236 * t40650 * t243 * t281;
    let t40664 = t10871 * t775;
    let t40672 = t10696 * t72;
    (t40639, t40649, t40650, t40654, t40664, t40672)
}
