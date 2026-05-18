//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 859/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk859<F: Float>(t8713: F, t938: F, t353: F, t4386: F, t2416: F, t891: F, t2367: F, t2503: F, t1114: F, t6744: F, t833: F, t4423: F) -> (F, F, F, F, F, F) {
    let t8714 = t8713 * t938;
    let t8715 = t353 * t8714;
    let t8716 = t4386 * t8715;
    let t8734 = t891 * t2416;
    let t8740 = F::new(7.0) / F::new(144.0) * t2367 * t2503;
    let t8743 = t1114 * t6744;
    let t8745 = F::new(7.0) / F::new(144.0) * t8743 * t833;
    let t8746 = t1114 * t4423;
    (t8716, t8734, t8740, t8743, t8745, t8746)
}
