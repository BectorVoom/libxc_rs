//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 605/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk605<F: Float>(t13546: F, t92: F, t13352: F, t2404: F, t13320: F, t3051: F, t13309: F, t13346: F, t683: F, t13301: F, t13296: F, t665: F, t668: F, t26: F, t2999: F, t13538: F, t13541: F, t13543: F, t13544: F, t9557: F, t9558: F, t9560: F, t9562: F, t9564: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13547 = t92 * t13546;
    let t13549 = t2404 * t13352;
    let t13550 = t92 * t13549;
    let t13552 = t2404 * t13320;
    let t13553 = t3051 * t13552;
    let t13555 = t2404 * t13309;
    let t13556 = t92 * t13555;
    let t13558 = t683 * t13346;
    let t13559 = t92 * t13558;
    let t13561 = t683 * t13301;
    let t13562 = t3051 * t13561;
    let t13564 = t683 * t13296;
    let t13565 = t92 * t13564;
    let t13567 = t665 * t668;
    let t13569 = t26 * t2999 * t13567;
    let t13571 = -t9557 - 8.0 / 27.0 * t9558 + 2.0 / 27.0 * t9560 - 2.0 / 9.0 * t9562 + t9564 / 9.0 - 4.0 / 27.0 * t13538 + t13541 - t13543 - 22.0 / 9.0 * t13544 - 10.0 / 27.0 * t13547 + 4.0 / 3.0 * t13550 + 8.0 / 9.0 * t13553 - 2.0 / 9.0 * t13556 - 2.0 * t13559 - 8.0 / 3.0 * t13562 + 2.0 / 3.0 * t13565 + 2.0 / 3.0 * t13569;
    (t13547, t13550, t13553, t13556, t13559, t13562, t13565, t13569, t13571)
}
