//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1130/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1130<F: Float>(t22632: F, t29510: F, t5598: F, t100634: F, t100850: F, t100880: F, t100910: F, t15658: F, t15677: F, t15689: F, t15802: F, t15812: F, t1603: F, t22522: F, t22767: F, t22777: F, t22826: F, t22834: F, t22842: F, t25734: F, t25759: F, t29469: F, t29515: F, t29546: F, t3052: F, t374: F, t5538: F, t73784: F, t92515: F, t92786: F, t92864: F, t929: F, t93268: F) -> (F,) {
    let t115790 = t5598 * t22632 * t29510;
    let t115810 = t100880 + 0.46509801892875584e-2 * t25734 * t15677 + 0.38731446812548799881e-3 * t25734 * t15658 - 0.11854761295685025975e-1 * t22842 * t73784 - 0.13784064983740990797e-3 * t5538 * t22777 * t29515 + 0.91830411319857336053e-5 * t5538 * t92786 * t29469 - 0.10214977340740740741e0 * t5598 * t22767 * t29510 + 0.12768721675925925926e-1 * t115790 - t100910 + 0.38731446812548799881e-3 * t22834 * t29546 + 0.38731446812548799881e-3 * t1603 * t92515 * t29469 + 0.51074886703703703704e-1 * t22522 * t100634 * t25759 * t3052 + 0.27568129967481981593e-3 * t92864 * t15802 - 0.93019603785751168e-1 * t93268 * t374 * t929 * t15812 - 0.93019603785751168e-2 * t22826 * t100850 * t15689;
    (t115810,)
}
