//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 969/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk969<F: Float>(t17182: F, t7263: F, t1773: F, t1769: F, t7203: F, t10872: F, t2464: F, t5032: F, t7261: F, t5038: F, t7262: F, t5030: F, t7268: F, t1785: F, t4999: F, t7208: F) -> (F, F, F, F, F, F) {
    let t17183 = t17182 * t7263;
    let t17184 = t1773 * t17183;
    let t17187 = 0.35981577432354634426e-1 * t7203 * t1769;
    let t17189 = t10872 * t2464 * t5032;
    let t17190 = t7261 * t17189;
    let t17193 = t7262 * t5038;
    let t17194 = t7261 * t17193;
    let t17197 = t5030 * t7268;
    let t17198 = t17197 * t1785;
    let t17199 = t7261 * t17198;
    let t17208 = 0.11993859144118211475e-1 * t7208 * t4999;
    (t17184, t17187, t17190, t17194, t17199, t17208)
}
