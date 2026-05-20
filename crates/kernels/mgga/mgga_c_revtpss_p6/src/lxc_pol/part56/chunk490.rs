//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 490/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk490<F: Float>(t1330: F, t72: F, t757: F, t525: F, t527: F, t2608: F, t520: F, t512: F, t19: F, t27: F, t521: F, t14: F, t22: F) -> (F, F, F, F, F, F) {
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3833 = F::new(1.0) / t525;
    let t3841 = F::new(1.0) / t527;
    let t3853 = t520 * t2608;
    let t3854 = t512 * t3853;
    let t3857 = t19 * t27;
    let t3859 = F::new(20.0) * t3857 * t521;
    let t3860 = t14 * t22;
    (t3826, t3833, t3841, t3854, t3859, t3860)
}
