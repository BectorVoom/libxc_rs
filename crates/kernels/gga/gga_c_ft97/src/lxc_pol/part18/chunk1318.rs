//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1318/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1318<F: Float>(t1570: F, t6615: F, t1559: F, t1969: F, t446: F, t6674: F, t8232: F, t18: F, t23900: F, t3281: F, t26768: F, t358: F, t363: F, t26888: F, t558: F, t9432: F) -> (F, F, F, F, F, F) {
    let t105462 = t6615 * t1570;
    let t105465 = t446 * t1969 * t105462 * t1559;
    let t105467 = t8232 * t6674;
    let t105468 = 4.0 / 27.0 * t105467;
    let t105471 = t3281 * t1969 * t23900 * t18;
    let t105473 = t26768 * t358;
    let t105476 = t446 * t1969 * t105473 * t363;
    let t105480 = t446 * t9432 * t26888 * t558;
    (t105465, t105467, t105468, t105471, t105476, t105480)
}
