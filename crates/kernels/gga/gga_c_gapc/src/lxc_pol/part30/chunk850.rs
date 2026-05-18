//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 850/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk850<F: Float>(t10146: F, t7371: F, t771: F, t2316: F, t3188: F, t284: F, t2902: F, t3216: F, t3218: F, t3231: F, t3243: F, t2786: F, t825: F) -> (F, F, F, F, F, F) {
    let t10147 = t10146 * t7371;
    let t10148 = t771 * t10147;
    let t10150 = t3188 * t2316;
    let t10151 = t284 * t10150;
    let t10153 = t2902 * t3216;
    let t10154 = t10153 * t3218;
    let t10156 = t3243 * t3231;
    let t10158 = t2786 * t825;
    (t10148, t10151, t10153, t10154, t10156, t10158)
}
