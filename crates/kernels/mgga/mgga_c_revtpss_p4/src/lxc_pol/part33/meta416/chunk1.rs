//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1485/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1485<F: Float>(t18547: F, t14363: F, t162: F, t18298: F, t187: F, t10563: F, t14324: F, t14343: F, t14345: F, t14372: F, t18535: F, t18536: F, t18537: F, t18538: F, t18541: F, t18543: F, t18546: F, t9394: F) -> (F, F, F, F) {
    let t18548 = F::new(8.0) * t18547;
    let t18549 = F::cast_from(0.21687162600603479684e-1_f64) * t14363;
    let t18550 = t18298 * t162;
    let t18552 = F::cast_from(0.19751673498613801407e-1_f64) * t18550 * t187;
    let t18553 = -t14324 + t18535 - t18536 - t18537 + t18538 + t14343 + t14345 + t18541 + t18543 + t18546 + t18548 + t9394 + t18549 + t18552 + t14372 + t10563;
    (t18548, t18549, t18552, t18553)
}
