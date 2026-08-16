//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1373/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1373<F: Float>(t3959: F, t9869: F, t2409: F, t35023: F, t3965: F, t13776: F, t3861: F, t3975: F, t9504: F, t3912: F, t50887: F, t14138: F) -> (F, F, F, F) {
    let t57595 = t3959 * t9869;
    let t57598 = t3965 * t2409 * t35023;
    let t57602 = t13776 * t3975 * t3861 * t9504;
    let t57604 = t3912 * t50887;
    let t57605 = t57604 * t14138;
    (t57595, t57598, t57602, t57605)
}
