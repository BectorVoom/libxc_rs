//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1276/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1276<F: Float>(t1196: F, t24765: F, t24255: F, t24257: F, t24259: F, t24261: F, t24482: F, t24484: F, t24490: F, t24496: F, t24500: F, t24763: F) -> (F, F) {
    let t24767 = F::cast_from(0.10254018858216406658e4_f64) * t1196 * t24765;
    let t24768 = t24490 + t24496 - t24500 + t24763 - t24767 - t24482 + t24255 - t24484 + t24257 + t24259 + t24261;
    (t24767, t24768)
}
