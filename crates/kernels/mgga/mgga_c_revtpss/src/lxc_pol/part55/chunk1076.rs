//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1076/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1076<F: Float>(t5: F, t33281: F, t8737: F, t32795: F, t32798: F, t32802: F, t32806: F, t33265: F, t33270: F, t33277: F, t8882: F, t117: F, t116: F, t8885: F) -> (F, F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t33283 = F::new(5.0) / F::new(27.0) * t8737 * t33281;
    let t33285 = piecewise3::<f64>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t32795 * t8882 + F::new(5.0) / F::new(12.0) * t32798 * t33265 + F::new(5.0) / F::new(18.0) * t32802 * t33270 - F::new(5.0) / F::new(72.0) * t32806 * t8882 - F::new(5.0) / F::new(36.0) * t8737 * t33277 + t33283);
    let t33286 = t33285 * t117;
    let t33287 = t8885 * t116;
    (t33283, t33285, t33286, t33287)
}
