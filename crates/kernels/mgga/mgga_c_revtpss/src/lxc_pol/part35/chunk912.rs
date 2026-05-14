//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 912/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk912<F: Float>(t5192: F, t6548: F, t12552: F, t24375: F, t12555: F, t1196: F, t24255: F, t24257: F, t24259: F, t24261: F, t24482: F, t24484: F, t24490: F, t24496: F, t24500: F, t24214: F, t24217: F, t24219: F, t24223: F, t24264: F, t24326: F, t24329: F, t24468: F, t24472: F, t24475: F, t24478: F, t24492: F) -> (F, F, F, F) {
    let t24763 = 0.35089341735807877242e1 * t5192 * t6548;
    let t24764 = t12552 * t24375;
    let t24765 = t24764 * t12555;
    let t24767 = 0.10254018858216406658e4 * t1196 * t24765;
    let t24768 = t24490 + t24496 - t24500 + t24763 - t24767 - t24482 + t24255 - t24484 + t24257 + t24259 + t24261;
    let t24769 = -t24264 + t24326 + t24329 - t24478 - t24492 + t24472 - t24468 - t24475 - t24219 + t24223 - t24214 + t24217;
    (t24763, t24767, t24768, t24769)
}
