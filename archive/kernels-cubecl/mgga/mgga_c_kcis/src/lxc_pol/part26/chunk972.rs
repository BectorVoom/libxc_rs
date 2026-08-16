//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 972/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk972<F: Float>(t22298: F, t589: F, t1505: F, t7267: F, t1555: F, t17308: F, t2069: F, t17311: F, t5900: F, t5897: F, t6048: F, t12338: F, t7271: F) -> (F, F, F, F, F, F, F) {
    let t22299 = t22298 * t589;
    let t22300 = t7267 * t1505;
    let t22301 = t22300 * t1555;
    let t22303 = F::cast_from(2.0_f64) * t17308 * t2069;
    let t22305 = F::cast_from(4.0_f64) * t17311 * t5900;
    let t22307 = F::cast_from(2.0_f64) * t5897 * t6048;
    let t22309 = F::cast_from(2.0_f64) * t12338 * t7271;
    (t22299, t22300, t22301, t22303, t22305, t22307, t22309)
}
