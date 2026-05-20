//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3332/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3332<F: Float>(t2: F, t4560: F, t580: F, t1587: F, t18890: F, t22: F, t4595: F, t52505: F, t4636: F, t52219: F, t15101: F, t15380: F) -> (F, F, F, F, F, F) {
    let t63202 = F::new(4.0) * t4560 * t2 * t580;
    let t63204 = F::new(2.0) * t1587 * t580;
    let t63206 = F::new(6.0) * t18890 * t22;
    let t63212 = F::new(8.0) * t52505 * t4595;
    let t63214 = F::cast_from(0.64327917994770140268e2_f64) * t52219 * t4636;
    let t63216 = F::new(8.0) * t15101 * t15380;
    (t63202, t63204, t63206, t63212, t63214, t63216)
}
