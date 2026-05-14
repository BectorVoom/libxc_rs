//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1090/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1090<F: Float>(t2715: F, t32105: F, t9434: F, t9442: F, t9439: F, t3783: F, t454: F, t1333: F, t9466: F, t9470: F, t9478: F, t20160: F, t9453: F, t9446: F, t9428: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32107 = 0.23148148148148148149e-2 * t2715 * t32105;
    let t32108 = t9434 * t9442;
    let t32115 = t9439 * t9442;
    let t32122 = t454 * t3783;
    let t32153 = t1333 * t9466;
    let t32155 = t1333 * t9470;
    let t32157 = t1333 * t9478;
    let t32173 = t20160 * t9453;
    let t32174 = t9446 * t32173;
    let t32176 = t20160 * t9428;
    (t32107, t32108, t32115, t32122, t32153, t32155, t32157, t32173, t32174, t32176)
}
