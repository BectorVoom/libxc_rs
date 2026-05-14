//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1076/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1076<F: Float>(t14682: F, t3140: F, t3989: F, t52921: F, t13796: F, t14423: F, t2195: F, t1118: F, t13859: F, t2190: F, t52926: F, t9284: F, t13972: F, t14726: F, t13808: F, t14588: F) -> (F, F, F, F, F, F) {
    let t52944 = t3989 * t14682 * t52921 * t3140;
    let t52952 = t3989 * t13796 * t14423 * t2195;
    let t52956 = t13859 * t13796 * t1118 * t2190;
    let t52959 = t13859 * t52926 * t9284;
    let t52961 = t13972 * t14726;
    let t52968 = t13808 * t14588;
    (t52944, t52952, t52956, t52959, t52961, t52968)
}
