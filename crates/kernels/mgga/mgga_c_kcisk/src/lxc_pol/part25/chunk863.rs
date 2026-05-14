//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 863/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk863<F: Float>(t15318: F, t2981: F, t15292: F, t15294: F, t15296: F, t15298: F, t15302: F, t15304: F, t15306: F, t15308: F, t880: F, t2977: F, t861: F, t73: F, t2980: F, t88: F) -> (F, F, F, F) {
    let t15319 = t15318 * t2981;
    let t15330 = -0.50638e1 * t15292 + 0.16879333333333333333e1 * t15294 - 0.19692555555555555555e1 * t15296 - 0.93011851851851851854e0 * t15298 + 0.27303333333333333333e0 * t15302 - 0.27303333333333333333e0 * t15304 - 0.3185388888888888889e0 * t15306 - 0.36514074074074074075e0 * t15308;
    let t15331 = t15330 * t880;
    let t15335 = 1.0 / t2977 / t861;
    let t15336 = t73 * t15335;
    let t15338 = 1.0 / t2980 / t88;
    (t15319, t15331, t15336, t15338)
}
