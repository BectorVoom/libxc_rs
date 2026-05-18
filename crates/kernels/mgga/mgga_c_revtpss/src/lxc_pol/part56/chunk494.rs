//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 494/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk494<F: Float>(t1362: F, t3920: F, t1386: F, t820: F, t843: F, t1401: F, t241: F, t1412: F, t72: F, t245: F) -> (F, F, F, F, F) {
    let t3922 = F::new(0.13009920719177044025e-1) * t1362 * t3920;
    let t3930 = t820 * t1386 * t843;
    let t3931 = t3930 * t1401;
    let t3934 = t820 * t1386 * t241;
    let t3935 = t1412 * t72;
    let t3936 = t3935 * t245;
    (t3922, t3930, t3931, t3934, t3936)
}
