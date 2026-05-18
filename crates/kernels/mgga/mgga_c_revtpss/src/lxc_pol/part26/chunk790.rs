//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 790/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk790<F: Float>(t10076: F, t10145: F, t1427: F, t1357: F, t4078: F, t689: F, t1445: F, t3899: F, t10115: F, t562: F, t2435: F, t3903: F) -> (F, F, F, F, F, F) {
    let t10146 = t10076 + t10145;
    let t10147 = t1427 * t10146;
    let t10150 = t1357 * t4078;
    let t10151 = t689 * t10150;
    let t10153 = t3899 * t1445;
    let t10154 = t689 * t10153;
    let t10157 = F::new(0.11044544084478153697e-3) * t10115 * t562;
    let t10160 = t2435 * t3903;
    (t10146, t10147, t10151, t10154, t10157, t10160)
}
