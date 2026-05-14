//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 871/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk871<F: Float>(t26538: F, t5778: F, t28: F, t23925: F, t6587: F, t1058: F, t558: F, t5769: F, t6580: F, t1349: F, t26515: F, t26521: F, t26524: F, t26527: F, t26529: F, t26531: F, t26535: F, t5766: F, t5781: F, t5845: F, t6589: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26539 = t5778 * t26538;
    let t26540 = t28 * t26539;
    let t26545 = t23925 * t6587;
    let t26546 = t28 * t26545;
    let t26551 = t1058 * t558;
    let t26552 = t5778 * t26551;
    let t26553 = t28 * t26552;
    let t26556 = t6580 * t5769;
    let t26558 = t1349 * t26515 / 6.0 + t6580 * t5845 / 6.0 + 4.0 * t26521 + 4.0 * t26524 + 4.0 * t26527 + 4.0 * t26529 + 4.0 * t26531 - t1349 * t26535 / 3.0 - t1349 * t26540 / 3.0 - t5766 * t6589 / 3.0 - t1349 * t26546 / 3.0 - t6580 * t5781 / 3.0 - t1349 * t26553 / 3.0 - t26556 / 18.0;
    (t26539, t26540, t26545, t26546, t26551, t26552, t26553, t26556, t26558)
}
