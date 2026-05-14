//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 702/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk702<F: Float>(t43: F, t5474: F, t85: F, t4030: F, t2635: F, t2835: F, t1690: F, t2898: F, t1694: F, t817: F, t1281: F, t234: F, t292: F, t5455: F, t822: F, t1699: F, t2910: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t5475 = t5474 * t85;
    let t5476 = 0.19751673498613801407e-1 * t5475;
    let t5477 = 0.48830526149350786811e-3 * t4030;
    let t5478 = 12.0 * t2635;
    let t5479 = 0.11696447245269292414e1 * t2835;
    let t5481 = t2898 * t1690;
    let t5486 = t817 * t1694;
    let t5492 = piecewise3(t44, 0.0, 8.0 / 27.0 * t5481 * t234 - 8.0 / 9.0 * t1281 * t822 - 2.0 / 9.0 * t5486 * t234 + 2.0 / 3.0 * t292 * t5455);
    let t5493 = t2910 * t1699;
    (t5475, t5476, t5477, t5478, t5479, t5481, t5486, t5492, t5493)
}
