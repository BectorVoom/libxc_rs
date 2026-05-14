//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 852/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk852<F: Float>(t19280: F, t824: F, t193: F, t89: F, t16579: F, t792: F, t666: F, t5225: F, t7640: F, t4056: F, t4129: F, t2336: F, t5221: F, t5217: F, t5209: F, t9725: F) -> (F, F, F, F, F, F, F) {
    let t19281 = t19280 * t824;
    let t19283 = t89 * t193 * t19281;
    let t19285 = t792 * t16579;
    let t19287 = t89 * t666 * t19285;
    let t19289 = t7640 * t5225;
    let t19290 = t19289 * t824;
    let t19292 = t89 * t193 * t19290;
    let t19293 = t4056 * t4129;
    let t19295 = t89 * t193 * t19293;
    let t19298 = t89 * t2336 * t5221;
    let t19301 = t89 * t2336 * t5217;
    let t19304 = t89 * t9725 * t5209;
    (t19283, t19287, t19292, t19295, t19298, t19301, t19304)
}
