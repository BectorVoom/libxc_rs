//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 548/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk548<F: Float>(t401: F, t77: F, t7983: F, t408: F, t428: F, t3020: F, t1609: F, t1593: F, t1608: F, t1615: F, t1630: F, t1711: F, t371: F, t407: F) -> (F, F, F, F, F, F, F) {
    let t7984 = t77 * t401;
    let t7985 = t7983 * t7984;
    let t7988 = t408 * t428;
    let t7989 = t3020 * t7988;
    let t8007 = t77 * t1609;
    let t8008 = t8007 * t1593;
    let t8009 = t1608 * t8008;
    let t8014 = t1615 * t1630;
    let t8015 = t1608 * t8014;
    let t8042 = t371 * t1711;
    let t8050 = t407 * t407;
    let t8051 = 1.0 / t8050;
    (t7985, t7989, t8009, t8015, t8042, t8050, t8051)
}
