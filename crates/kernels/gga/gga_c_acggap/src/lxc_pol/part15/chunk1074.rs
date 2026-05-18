//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1074/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1074<F: Float>(t2385: F, t323: F, t851: F, t7990: F, t9154: F, t862: F, t865: F, t32092: F, t9168: F, t33323: F, t557: F, t33092: F) -> (F, F, F, F, F, F) {
    let t38285 = t851 * t2385 * t323;
    let t38293 = F::new(0.34694512752820797848e1) * t7990 * t9154;
    let t38309 = t862 * t2385 * t865;
    let t38315 = F::new(0.17347256376410398924e1) * t32092 * t9168;
    let t38319 = F::new(0.13170898365871023197e1) * t33323 * t557;
    let t38321 = t33092 * t557;
    (t38285, t38293, t38309, t38315, t38319, t38321)
}
