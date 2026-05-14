//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 721/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk721<F: Float>(t458: F, t4772: F, t16919: F, t24: F, t586: F, t16708: F, t2102: F, t16719: F, t9192: F, t15742: F, t3499: F, t12809: F, t12816: F, t12834: F, t12836: F, t12839: F, t12850: F, t17256: F, t17261: F, t17265: F, t17268: F, t17272: F, t17274: F, t17276: F, t17279: F, t3139: F, t462: F, t9179: F, t92: F) -> (F,) {
    let t17281 = t458 * t4772;
    let t17284 = t24 * t586 * t16919;
    let t17286 = t2102 * t16708;
    let t17289 = t9192 * t16719;
    let t17292 = t3499 * t15742;
    let t17295 = 2.0 * t462 * t17256 - 6.0 * t462 * t17261 + 4.0 * t462 * t17265 - t462 * t17268 / 3.0 - 8.0 / 9.0 * t12809 + t12816 - 2.0 / 9.0 * t17272 + t17274 / 9.0 + 2.0 / 27.0 * t17276 - t12834 - t12836 + t12839 - t12850 - 4.0 / 9.0 * t9179 + t17279 / 3.0 - 2.0 / 3.0 * t17281 - t92 * t17284 + 4.0 / 3.0 * t3139 * t17286 + 2.0 / 9.0 * t462 * t17289 + 4.0 / 3.0 * t462 * t17292;
    (t17295,)
}
