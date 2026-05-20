//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1943/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1943<F: Float>(t670: F, t7373: F, t101451: F, t101453: F, t101455: F, t101458: F, t101461: F, t101464: F, t101466: F, t94976: F, t94979: F, t94981: F, t95397: F) -> (F, F) {
    let t101725 = t670 * t7373;
    let t101754 = F::new(22.0) / F::new(9.0) * t101451;
    let t101755 = F::new(8.0) / F::new(3.0) * t101453;
    let t101756 = F::new(4.0) / F::new(3.0) * t101455;
    let t101760 = -t95397 - F::new(44.0) / F::new(9.0) * t94976 - F::new(4.0) / F::new(3.0) * t94979 + F::new(2.0) / F::new(3.0) * t94981 - t101754 - t101755 + t101756 - F::new(3.0) / F::new(2.0) * t101458 + t101461 + t101464 / F::new(2.0) - t101466 / F::new(4.0);
    (t101725, t101760)
}
