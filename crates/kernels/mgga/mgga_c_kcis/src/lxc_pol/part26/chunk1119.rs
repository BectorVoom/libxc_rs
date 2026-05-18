//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1119/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1119<F: Float>(t573: F, t5998: F, t27517: F, t8196: F, t1468: F, t5929: F, t2062: F, t3738: F, t5910: F, t7952: F, t27543: F, t4122: F) -> (F, F, F, F, F, F) {
    let t28614 = t5998 * t573;
    let t28616 = t27517 * t8196;
    let t28618 = t1468 * t5929;
    let t28620 = t3738 * t2062;
    let t28622 = t7952 * t5910;
    let t28624 = t4122 * t27543;
    (t28614, t28616, t28618, t28620, t28622, t28624)
}
