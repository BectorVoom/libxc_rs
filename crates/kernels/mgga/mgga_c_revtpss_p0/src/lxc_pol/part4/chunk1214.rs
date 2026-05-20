//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1214/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1214<F: Float>(t14338: F, t14381: F, t14435: F, t15081: F, t2: F, t895: F, t580: F, t265: F, t22: F, t4567: F, t1610: F, t2875: F) -> (F, F, F, F, F) {
    let t15083 = t14338 + t14381 + t14435 + t15081;
    let t15091 = t895 * t2;
    let t15093 = F::new(2.0) * t15091 * t580;
    let t15094 = t265 * t580;
    let t15096 = F::new(3.0) * t4567 * t22;
    let t15098 = t1610 * t2875;
    (t15083, t15093, t15094, t15096, t15098)
}
