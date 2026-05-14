//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 590/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk590<F: Float>(t1225: F, t2258: F, t1012: F, t1224: F, t3367: F, t2251: F, t1121: F, t404: F) -> (F, F, F, F, F) {
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    let t3692 = t1224 * t3367;
    let t3693 = t3692 * t2251;
    let t3694 = t1012 * t3693;
    let t3698 = 1.0 / t404 / t1121;
    (t3688, t3689, t3693, t3694, t3698)
}
