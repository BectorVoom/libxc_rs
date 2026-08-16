//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3632/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3632<F: Float>(t12227: F, t3385: F, t6474: F, t16942: F, t1733: F, t3384: F, t12248: F, t3427: F, t20651: F, t44017: F, t6471: F, t20644: F) -> (F, F, F, F, F, F) {
    let t68754 = F::cast_from(0.57895126195293126241e3_f64) * t12227 * t6474 * t3385;
    let t68757 = F::cast_from(4.0_f64) * t3384 * t1733 * t16942;
    let t68760 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t6474 * t3427;
    let t68763 = F::cast_from(0.62071215503128080361e4_f64) * t44017 * t20651 * t3385;
    let t68766 = F::cast_from(2.0_f64) * t3384 * t6471 * t3427;
    let t68769 = F::cast_from(0.96491876992155210402e2_f64) * t12248 * t20644 * t3385;
    (t68754, t68757, t68760, t68763, t68766, t68769)
}
