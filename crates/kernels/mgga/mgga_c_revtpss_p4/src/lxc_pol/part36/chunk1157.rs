//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1157/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1157<F: Float>(t27955: F, t5273: F, t7617: F, t5291: F, t7616: F, t1241: F, t5265: F, t7618: F, t1219: F, t8172: F, t5357: F, t7607: F) -> (F, F, F, F, F, F, F) {
    let t28885 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t27955;
    let t29010 = t5273 * t7617;
    let t29019 = t7616 * t5291;
    let t29020 = t1241 * t29019;
    let t29023 = t7618 * t5265;
    let t29027 = t8172 * t1219;
    let t29031 = t7607 * t5357;
    (t28885, t29010, t29019, t29020, t29023, t29027, t29031)
}
