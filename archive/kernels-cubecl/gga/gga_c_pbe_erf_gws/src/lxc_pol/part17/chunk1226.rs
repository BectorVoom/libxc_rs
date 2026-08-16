//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1226/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1226<F: Float>(t13796: F, t13859: F, t3097: F, t875: F, t14682: F, t3140: F, t3989: F, t52921: F, t14423: F, t2195: F, t1118: F, t2190: F) -> (F, F, F, F) {
    let t52940 = t13859 * t13796 * t3097 * t875;
    let t52944 = t3989 * t14682 * t52921 * t3140;
    let t52952 = t3989 * t13796 * t14423 * t2195;
    let t52956 = t13859 * t13796 * t1118 * t2190;
    (t52940, t52944, t52952, t52956)
}
