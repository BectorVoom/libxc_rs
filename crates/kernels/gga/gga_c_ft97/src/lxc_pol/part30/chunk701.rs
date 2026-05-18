//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 701/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk701<F: Float>(t684: F, t7114: F, t15312: F, t24873: F, t4255: F, t10703: F, t11593: F, t1901: F, t29147: F, t29151: F, t29155: F, t29158: F, t29162: F, t29166: F, t29170: F, t29174: F, t29178: F, t29182: F, t29186: F, t3281: F, t446: F) -> (F, F, F) {
    let t29189 = t7114 * t684;
    let t29190 = t15312 * t29189;
    let t29193 = t24873 * t4255;
    let t29194 = t10703 * t29193;
    let t29197 = t1901 * t29147 / F::new(9.0) + t1901 * t29151 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t11593 * t29155 - t446 * t29158 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t3281 * t29162 - t446 * t29166 / F::new(9.0) - t446 * t29170 / F::new(3.0) - t446 * t29174 / F::new(3.0) - t446 * t29178 / F::new(3.0) - t446 * t29182 / F::new(3.0) - t1901 * t29186 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t29190 - t1901 * t29194 / F::new(9.0);
    (t29189, t29193, t29197)
}
