//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 973/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk973<F: Float>(t10556: F, t544: F, t2392: F, t10506: F, t10508: F, t10510: F, t10512: F, t10516: F, t10519: F, t10522: F, t10529: F, t10536: F, t10539: F, t10542: F, t10545: F, t10549: F, t10551: F, t10554: F) -> (F, F) {
    let t10557 = t544 * t10556;
    let t10559 = F::new(0.42900587942220512003e1) * t10557 * t2392;
    let t10560 = t10506 + t10508 + t10510 + t10512 - t10516 + t10519 - t10522 - t10529 + t10536 + t10539 + t10542 + t10545 - t10549 - t10551 + t10554 + t10559;
    (t10557, t10560)
}
