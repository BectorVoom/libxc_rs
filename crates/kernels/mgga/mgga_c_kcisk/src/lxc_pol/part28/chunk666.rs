//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 666/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk666<F: Float>(t776: F, t2028: F, t7552: F, t41: F, t6884: F, t260: F, t604: F, t67: F, t4971: F, t6707: F, t1758: F, t1995: F, t2442: F, t2620: F, t525: F, t642: F, t79: F) -> (F, F, F, F, F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t7553 = t7552 * t2028;
    let t7556 = t6884 * t41;
    let t7567 = t260 * t67 * t604;
    let t7568 = t41 * t4971;
    let t7569 = t7568 * t6707;
    let t7573 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t7556 * t642 - 10.0 / 27.0 * t525 * t2620 * t1758 - 10.0 / 27.0 * t525 * t1995 * t2442 + 40.0 / 81.0 * t7567 * t7569);
    let t7574 = t79 * t7573;
    (t7553, t7556, t7567, t7568, t7569, t7573, t7574)
}
