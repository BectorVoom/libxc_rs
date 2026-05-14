//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1226/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1226<F: Float>(t2666: F, t9772: F, t2815: F, t7690: F, t10039: F, t2049: F, t34314: F, t34317: F, t34319: F, t34322: F, t34325: F, t34327: F, t34330: F, t34332: F, t34334: F, t34336: F, t34338: F, t34340: F, t34342: F) -> (F, F, F, F) {
    let t34612 = t9772 * t2666;
    let t34615 = t2815 * t7690;
    let t34618 = t10039 * t2049;
    let t34635 = -0.9375e-1 * t34314 + 0.625e-1 * t34317 + 0.26979166666666666667e-1 * t34319 - 0.89930555555555555557e-2 * t34322 - 0.9375e-1 * t34325 + 0.625e-1 * t34327 + 0.25e0 * t34330 + 0.14388888888888888889e0 * t34332 + 0.20234375e-1 * t34334 - 0.20234375e-1 * t34336 + 0.10791666666666666667e0 * t34338 - 0.10791666666666666667e0 * t34340 + 0.20234375e-1 * t34342;
    (t34612, t34615, t34618, t34635)
}
