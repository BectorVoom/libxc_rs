//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1138/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1138<F: Float>(t14422: F, t285: F, t25: F, t4973: F, t291: F, t992: F, t330: F, t737: F, t14394: F, t14397: F, t14404: F, t14409: F, t14415: F, t4974: F, t9608: F, t9611: F, t9614: F, t9620: F, t9623: F, t984: F) -> F {
    let t14423 = t285 * t14422;
    let t14425 = t25 * t4973;
    let t14427 = t285 * t14425 / F::new(144.0);
    let t14430 = t992 * t291;
    let t14431 = t14430 * t330;
    let t14432 = t737 * t14431;
    let t14435 = F::new(11.0) / F::new(324.0) * t9608 + t14394 * t14397 / F::new(72.0) + t14394 * t14404 / F::new(72.0) - t14394 * t14409 / F::new(108.0) - t285 * t14415 / F::new(96.0) + t9611 / F::new(144.0) + t9614 / F::new(216.0) + t9620 / F::new(54.0) - t9623 / F::new(288.0) + t14423 / F::new(432.0) - t14427 + t984 * t4974 / F::new(18.0) + t285 * t14432 / F::new(144.0);
    t14435
}
