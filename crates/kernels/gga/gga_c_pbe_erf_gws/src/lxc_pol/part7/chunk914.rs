//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 914/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk914<F: Float>(t17188: F, t587: F, t1897: F, t4991: F, t1634: F, t5463: F, t639: F, t155: F, t1639: F, t1644: F, t1648: F, t5288: F) -> (F, F, F, F, F) {
    let t17189 = t587 * t17188;
    let t17190 = F::new(128.0) / F::new(1215.0) * t17189;
    let t17192 = t587 * t4991 * t1897;
    let t17193 = F::new(32.0) / F::new(135.0) * t17192;
    let t17195 = t639 * t5463 * t1634;
    let t17196 = F::new(16.0) / F::new(135.0) * t17195;
    let t17197 = t155 * t1639;
    let t17199 = t639 * t17197 * t1644;
    let t17200 = F::new(16.0) / F::new(81.0) * t17199;
    let t17202 = F::new(16.0) / F::new(45.0) * t1648 * t5288;
    (t17190, t17193, t17196, t17200, t17202)
}
