//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 881/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk881<F: Float>(t1158: F, t6505: F, t2344: F, t904: F, t1150: F, t6717: F, t2246: F, t3099: F, t3202: F, t840: F, t2376: F, t3306: F) -> (F, F, F, F, F, F) {
    let t9658 = t6505 * t1158;
    let t9665 = t2344 * t904;
    let t9669 = t6717 * t1150;
    let t9695 = F::new(7.0) / F::new(72.0) * t2246 * t3099;
    let t9701 = F::new(7.0) / F::new(144.0) * t840 * t3202;
    let t9707 = t2376 * t3306;
    (t9658, t9665, t9669, t9695, t9701, t9707)
}
