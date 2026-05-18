//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 905/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk905<F: Float>(t31781: F, t31784: F, t1949: F, t1955: F, t233: F, t25373: F, t886: F, t31777: F) -> (F, F, F, F) {
    let t31786 = F::new(0.25702851531048074406e-1) * t31784 * t31781;
    let t31787 = t1955 * t1949;
    let t31791 = t25373 * t233 * t886;
    let t31794 = t1955 * t31777;
    (t31786, t31787, t31791, t31794)
}
