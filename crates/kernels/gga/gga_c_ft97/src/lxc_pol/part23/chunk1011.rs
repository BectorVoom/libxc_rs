//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1011/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1011<F: Float>(t4635: F, t6135: F, t2354: F, t446: F, t24519: F, t4917: F, t9744: F, t24483: F, t27466: F, t27473: F, t27792: F, t30960: F, t30964: F, t30968: F, t30973: F, t30976: F, t30980: F, t30984: F, t30988: F, t30993: F, t30998: F) -> (F, F, F, F, F) {
    let t31000 = t6135 * t4635;
    let t31001 = t2354 * t31000;
    let t31002 = t446 * t31001;
    let t31004 = t24519 * t4917;
    let t31005 = t9744 * t31004;
    let t31006 = t446 * t31005;
    let t31008 = t27466 / 3.0 - 2.0 / 9.0 * t27473 + t30960 / 3.0 + t30964 / 6.0 + t30968 / 9.0 - t30973 - 2.0 / 3.0 * t30976 - t30980 / 6.0 - t24483 - t30984 / 3.0 - 2.0 / 3.0 * t30988 - 3.0 / 8.0 * t30993 - t27792 / 9.0 - t30998 / 2.0 + t31002 / 3.0 + 2.0 / 9.0 * t31006;
    (t31001, t31002, t31005, t31006, t31008)
}
