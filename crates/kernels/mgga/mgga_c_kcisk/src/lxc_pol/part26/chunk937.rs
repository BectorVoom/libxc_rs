//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 937/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk937<F: Float>(t12929: F, t12975: F, t19100: F, t19103: F, t19105: F, t19106: F, t25590: F, t25593: F, t25596: F, t25599: F, t25601: F, t25604: F, t25607: F, t25609: F, t25612: F, t25615: F, t25618: F) -> (F,) {
    let t25620 = -t12975 - 4.0 / 27.0 * t12929 - 8.0 / 27.0 * t19100 + t19103 - t19105 + 4.0 / 9.0 * t19106 + 2.0 / 27.0 * t25590 - 10.0 / 27.0 * t25593 + 4.0 / 3.0 * t25596 - 8.0 / 9.0 * t25599 - 2.0 / 9.0 * t25601 - 2.0 * t25604 + 8.0 / 3.0 * t25607 + t25609 / 9.0 - 2.0 / 9.0 * t25612 + 2.0 / 3.0 * t25615 - t25618 / 3.0;
    (t25620,)
}
