//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1331/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1331<F: Float>(t17311: F, t28573: F, t4189: F, t6048: F, t8207: F, t12338: F, t29430: F, t1628: F, t29624: F, t2069: F, t28644: F, t2253: F, t22714: F) -> (F, F, F, F, F, F) {
    let t102813 = F::cast_from(4.0_f64) * t17311 * t28573;
    let t102816 = F::cast_from(4.0_f64) * t4189 * t8207 * t6048;
    let t102820 = F::cast_from(2.0_f64) * t12338 * t29430;
    let t102823 = t29624 * t1628;
    let t102828 = F::cast_from(4.0_f64) * t4189 * t28644 * t2069;
    let t102833 = F::cast_from(2.0_f64) * t4189 * t2253 * t22714;
    (t102813, t102816, t102820, t102823, t102828, t102833)
}
