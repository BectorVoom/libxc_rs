//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1113/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1113<F: Float>(t6048: F, t7940: F, t17308: F, t2253: F, t17311: F, t7943: F, t5897: F, t7962: F, t12338: F, t8186: F, t1555: F, t12345: F) -> (F, F, F, F, F, F, F) {
    let t28563 = t7940 * t6048;
    let t28564 = t17308 * t2253;
    let t28566 = F::cast_from(2.0_f64) * t17311 * t7943;
    let t28567 = t5897 * t7962;
    let t28569 = F::cast_from(2.0_f64) * t12338 * t8186;
    let t28570 = t8186 * t1555;
    let t28572 = F::cast_from(6.0_f64) * t12345 * t28570;
    (t28563, t28564, t28566, t28567, t28569, t28570, t28572)
}
