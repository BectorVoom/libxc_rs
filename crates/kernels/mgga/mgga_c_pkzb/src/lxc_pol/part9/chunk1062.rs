//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1062/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1062<F: Float>(t2667: F, t5296: F, t5257: F, t6941: F, t1721: F, t568: F, t1719: F, t2639: F, t5391: F, t17051: F, t175: F, t2590: F, t2595: F, t17043: F, t6888: F, t17053: F, t2602: F) -> (F, F, F, F, F, F, F, F) {
    let t20164 = t5296 * t2667;
    let t20166 = t5257 * t6941;
    let t20168 = t1721 * t568;
    let t20195 = t2639 * t5391 * t1719;
    let t20199 = t17051 * t175;
    let t20201 = t2590 * t20199 * t2595;
    let t20202 = 0.34013387707001991332e-1 * t20201;
    let t20203 = t17043 * t6888;
    let t20205 = t17053 * t2602;
    (t20164, t20166, t20168, t20195, t20199, t20202, t20203, t20205)
}
