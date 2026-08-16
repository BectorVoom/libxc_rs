//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 949/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk949<F: Float>(t14422: F, t285: F, t25: F, t4973: F, t291: F, t992: F, t330: F, t737: F, t14394: F, t14397: F, t14404: F, t14409: F, t14415: F, t4974: F, t9608: F, t9611: F, t9614: F, t9620: F, t9623: F, t984: F) -> (F, F) {
    let t14423 = t285 * t14422;
    let t14425 = t25 * t4973;
    let t14427 = t285 * t14425 / F::cast_from(144.0_f64);
    let t14430 = t992 * t291;
    let t14431 = t14430 * t330;
    let t14432 = t737 * t14431;
    let t14435 = F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t9608 + t14394 * t14397 / F::cast_from(72.0_f64) + t14394 * t14404 / F::cast_from(72.0_f64) - t14394 * t14409 / F::cast_from(108.0_f64) - t285 * t14415 / F::cast_from(96.0_f64) + t9611 / F::cast_from(144.0_f64) + t9614 / F::cast_from(216.0_f64) + t9620 / F::cast_from(54.0_f64) - t9623 / F::cast_from(288.0_f64) + t14423 / F::cast_from(432.0_f64) - t14427 + t984 * t4974 / F::cast_from(18.0_f64) + t285 * t14432 / F::cast_from(144.0_f64);
    (t14430, t14435)
}
