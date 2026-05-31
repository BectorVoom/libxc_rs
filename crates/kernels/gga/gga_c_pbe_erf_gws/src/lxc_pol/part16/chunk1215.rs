//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1215/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1215<F: Float>(t14240: F, t2376: F, t829: F, t830: F, t14327: F, t2367: F, t14243: F, t840: F, t51869: F, t1206: F, t2074: F, t353: F, t4386: F) -> (F, F, F, F, F) {
    let t52478 = t2376 * t14240;
    let t52480 = t829 * t830 * t52478;
    let t52483 = t2367 * t14327;
    let t52514 = t840 * t14243;
    let t52525 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t51869;
    let t52529 = t4386 * t353 * t1206 * t2074;
    (t52480, t52483, t52514, t52525, t52529)
}
