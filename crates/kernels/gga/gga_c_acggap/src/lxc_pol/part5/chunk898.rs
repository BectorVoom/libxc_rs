//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 898/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk898<F: Float>(t168: F, t352: F, t355: F, t4353: F, t721: F, t4795: F, t4818: F, t4822: F, t13768: F, t2: F, t3153: F, t495: F, t1476: F, t3143: F, t1049: F, t4833: F) -> (F, F, F, F, F, F, F) {
    let t16236 = t352 * t168 * t355;
    let t16238 = t16236 * t4353 * t721;
    let t16241 = t4795 * t4818 * t721;
    let t16244 = t4795 * t4822 * t721;
    let t16249 = t3153 * t13768 * t495 * t2;
    let t16253 = t3143 * t1476;
    let t16255 = t1049 * t4833;
    (t16236, t16238, t16241, t16244, t16249, t16253, t16255)
}
