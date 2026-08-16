//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1063/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1063<F: Float>(t2344: F, t904: F, t8828: F, t1150: F, t6717: F, t8886: F, t3219: F, t3235: F, t6360: F, t875: F, t9375: F, t2343: F, t3247: F, t6714: F, t6718: F, t9181: F, t9183: F, t9187: F, t9190: F, t9192: F, t9196: F) -> (F, F, F, F, F) {
    let t9665 = t2344 * t904;
    let t9666 = t9665 * t8828;
    let t9669 = t6717 * t1150;
    let t9671 = t9665 * t8886;
    let t9675 = t3235 * t3219 * t6360;
    let t9681 = t3235 * t9375 * t875;
    let t9684 = t2343 * t9666 / F::cast_from(192.0_f64) + F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t9669 + t9181 - t3247 * t9671 / F::cast_from(64.0_f64) + t9183 + t9187 + t3247 * t9675 / F::cast_from(512.0_f64) - t9190 - t9192 - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t6714 + F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t6718 - t9196 - t2343 * t9681 / F::cast_from(768.0_f64);
    (t9666, t9671, t9675, t9681, t9684)
}
