//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 980/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk980<F: Float>(t2409: F, t3068: F, t8734: F, t2367: F, t2503: F, t1114: F, t6744: F, t833: F, t4423: F, t3199: F, t376: F, t829: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t8736 = t2409 * t8734 * t3068;
    let t8740 = F::new(7.0) / F::new(144.0) * t2367 * t2503;
    let t8743 = t1114 * t6744;
    let t8745 = F::new(7.0) / F::new(144.0) * t8743 * t833;
    let t8746 = t1114 * t4423;
    let t8747 = t8746 * t833;
    let t8749 = t3199 * t376;
    let t8751 = t829 * t830 * t8749;
    (t8736, t8740, t8743, t8745, t8746, t8747, t8749, t8751)
}
