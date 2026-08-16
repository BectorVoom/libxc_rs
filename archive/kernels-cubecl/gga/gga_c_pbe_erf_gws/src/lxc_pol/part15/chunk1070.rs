//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1070/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1070<F: Float>(t810: F, t944: F, t2074: F, t3944: F, t2376: F, t4052: F, t829: F, t830: F) -> (F, F, F) {
    let t13763 = t810 * t944;
    let t13767 = t3944 * t2074;
    let t13770 = t2376 * t4052;
    let t13772 = t829 * t830 * t13770;
    (t13763, t13767, t13772)
}
