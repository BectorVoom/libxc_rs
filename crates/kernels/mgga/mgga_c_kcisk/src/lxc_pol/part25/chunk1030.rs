//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1030/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1030<F: Float>(t776: F, t11153: F, t41: F, t17132: F, t16617: F, t7568: F, t1758: F, t18289: F, t18306: F, t2442: F, t2620: F, t4973: F, t4977: F, t525: F, t5449: F, t642: F, t7556: F, t7567: F, t7569: F) -> (F,) {
    let t777 = t776 < -0.66725e-1;
    let t18309 = t41 * t11153;
    let t18310 = t18309 * t17132;
    let t18313 = t7568 * t16617;
    let t18317 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t18289 * t642 - 20.0 / 27.0 * t525 * t7556 * t1758 + 40.0 / 81.0 * t525 * t2620 * t4973 - 10.0 / 27.0 * t525 * t2620 * t4977 - 10.0 / 27.0 * t525 * t5449 * t2442 + 80.0 / 81.0 * t18306 * t7569 - 280.0 / 243.0 * t7567 * t18310 + 40.0 / 81.0 * t7567 * t18313);
    (t18317,)
}
