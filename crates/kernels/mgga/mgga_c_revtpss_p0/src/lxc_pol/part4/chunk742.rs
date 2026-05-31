//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 742/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk742<F: Float>(t3347: F, t3812: F, t1312: F, t2320: F, t2322: F, t2327: F, t2371: F, t670: F, t93: F, t1330: F, t72: F, t757: F) -> (F, F, F, F) {
    let t3813 = t3347 + t3812;
    let t3821 = F::cast_from(2.0_f64) * t1312 * t2371 + F::cast_from(4.0_f64) * t2322 * t670 + F::cast_from(2.0_f64) * t2327 * t93 + t2320;
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    (t3813, t3821, t3825, t3826)
}
