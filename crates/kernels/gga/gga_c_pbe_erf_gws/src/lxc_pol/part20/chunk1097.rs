//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1097/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1097<F: Float>(t14652: F, t4414: F, t14127: F, t2503: F, t14803: F, t4182: F, t6781: F, t829: F, t830: F, t13791: F, t3039: F, t1144: F, t4387: F, t859: F, t14420: F, t19906: F) -> (F, F, F, F, F, F, F) {
    let t53636 = 7.0 / 36.0 * t4414 * t14652;
    let t53645 = t14127 * t2503;
    let t53646 = 7.0 / 144.0 * t53645;
    let t53656 = 7.0 / 36.0 * t4414 * t14803;
    let t53679 = t6781 * t4182;
    let t53681 = t829 * t830 * t53679;
    let t53688 = t3039 * t13791;
    let t53699 = t859 * t1144 * t4387;
    let t53704 = 7.0 / 72.0 * t19906 * t14420;
    (t53636, t53646, t53656, t53681, t53688, t53699, t53704)
}
