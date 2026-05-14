//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 999/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk999<F: Float>(t17311: F, t7943: F, t5897: F, t7962: F, t12338: F, t8186: F, t1555: F, t12345: F, t2069: F, t4189: F, t2253: F, t6048: F, t4184: F, t8207: F, t1468: F, t6034: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28566 = 2.0 * t17311 * t7943;
    let t28567 = t5897 * t7962;
    let t28569 = 2.0 * t12338 * t8186;
    let t28570 = t8186 * t1555;
    let t28572 = 6.0 * t12345 * t28570;
    let t28573 = t7962 * t2069;
    let t28575 = 2.0 * t4189 * t28573;
    let t28576 = t2253 * t6048;
    let t28578 = 2.0 * t4189 * t28576;
    let t28579 = t4184 * t8207;
    let t28580 = t8207 * t1555;
    let t28582 = 2.0 * t4189 * t28580;
    let t28583 = t1468 * t6034;
    (t28566, t28567, t28569, t28570, t28572, t28573, t28575, t28576, t28578, t28579, t28580, t28582, t28583)
}
