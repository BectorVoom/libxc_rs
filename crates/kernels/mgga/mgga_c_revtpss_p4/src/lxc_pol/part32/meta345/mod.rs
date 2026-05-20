//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1273;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta345<F: Float>(t14054: F, t3992: F, t2661: F, t5774: F, t72: F, t686: F, t3915: F, t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t2470: F, t5721: F, t1445: F, t5599: F, t2435: F, t5600: F, t1426: F, t1893: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14057, t14079, t14081, t14084, t14085) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1273::<F>(t14054, t3992, t2661, t5774, t72, t686, t3915, t5711, t786, t1364, t1357, t5775);
        let (t14087, t14090, t14091, t14096, t14097, t14100) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1274::<F>(t14085, t689, t2470, t5721, t3915, t1445, t5599, t2435, t5600, t1426, t1893, t786);
    (t14057, t14079, t14081, t14084, t14087, t14090, t14091, t14096, t14097, t14100)
}
