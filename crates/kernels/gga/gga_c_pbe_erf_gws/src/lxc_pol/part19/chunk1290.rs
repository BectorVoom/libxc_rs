//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1290/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1290<F: Float>(t13859: F, t14682: F, t56296: F, t6287: F, t15161: F, t2397: F, t12074: F, t3079: F, t14135: F, t3912: F, t51913: F, t11505: F, t3972: F, t3975: F) -> (F, F, F, F, F) {
    let t56586 = t13859 * t14682 * t56296 * t6287;
    let t56588 = t15161 * t2397;
    let t56590 = t12074 * t3079;
    let t56593 = t3912 * t14135 * t51913;
    let t56596 = t3972 * t3975 * t11505;
    (t56586, t56588, t56590, t56593, t56596)
}
