//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 846/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk846<F: Float>(t25296: F, t7058: F, t2453: F, t7057: F, t136: F, t1958: F, t2457: F, t1954: F, t9645: F) -> (F, F, F, F, F) {
    let t25297 = t7058 * t25296;
    let t25299 = t2453 * t7057;
    let t25300 = t1958 * t136;
    let t25301 = t25300 * t2457;
    let t25303 = F::new(0.17135234354032049604e-2) * t25299 * t25301;
    let t25304 = t1954 * t9645;
    (t25297, t25299, t25301, t25303, t25304)
}
