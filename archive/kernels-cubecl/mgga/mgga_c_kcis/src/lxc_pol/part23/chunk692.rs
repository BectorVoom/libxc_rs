//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 692/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk692<F: Float>(t2257: F, t7974: F, t1592: F, t251: F, t1598: F) -> (F, F, F) {
    let t7976 = F::cast_from(0.11584201388888888889e-3_f64) * t2257 * t7974;
    let t7977 = t1592 * t251;
    let t7978 = t7977 * t1598;
    (t7976, t7977, t7978)
}
