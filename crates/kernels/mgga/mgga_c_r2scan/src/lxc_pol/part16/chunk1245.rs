//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1245/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1245<F: Float>(t10680: F, t10681: F, t10683: F, t3033: F, t10673: F, t10674: F, t10676: F, t39290: F, t42462: F, t42465: F, t42467: F, t42471: F, t42475: F, t42814: F, t42818: F, t42822: F, t42824: F, t42826: F, t42832: F, t42836: F) -> F {
    let t43838 = t10680 * t10681 * t3033 * t10683;
    let t43842 = t10673 * t10674 * t3033 * t10676;
    let t43844 = -t42462 + t42465 + t42467 - t42471 + t42475 + t42814 - t42818 - t42822 - t42824 + t42826 + F::new(0.60975299583150056628e-3) * t39290 - t42832 - t42836 + F::new(0.36021158228745895953e-3) * t43838 - F::new(0.5124043883133942371e-4) * t43842;
    t43844
}
