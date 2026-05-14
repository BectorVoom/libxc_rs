//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1187/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1187<F: Float>(t17311: F, t28573: F, t4189: F, t6048: F, t8207: F, t12338: F, t29430: F, t1628: F, t29624: F, t2069: F, t28644: F, t2253: F, t22714: F, t12345: F, t8186: F, t1555: F) -> (F, F, F, F, F, F, F, F) {
    let t102813 = 4.0 * t17311 * t28573;
    let t102816 = 4.0 * t4189 * t8207 * t6048;
    let t102820 = 2.0 * t12338 * t29430;
    let t102823 = t29624 * t1628;
    let t102828 = 4.0 * t4189 * t28644 * t2069;
    let t102833 = 2.0 * t4189 * t2253 * t22714;
    let t102836 = 12.0 * t12345 * t8186 * t6048;
    let t102839 = 6.0 * t12345 * t29430 * t1555;
    (t102813, t102816, t102820, t102823, t102828, t102833, t102836, t102839)
}
