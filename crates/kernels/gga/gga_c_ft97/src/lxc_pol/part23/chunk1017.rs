//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1017/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1017<F: Float>(t31060: F, t762: F, t1449: F, t5064: F, t10052: F, t24628: F, t27466: F, t27473: F, t27792: F, t30960: F, t30964: F, t30968: F, t30973: F, t30976: F, t30980: F, t30984: F, t30988: F, t30993: F, t30998: F, t31002: F, t31006: F) -> (F, F, F, F) {
    let t31061 = t762 * t31060;
    let t31063 = t1449 * t5064;
    let t31064 = t10052 * t31063;
    let t31081 = t27466 / 9.0 - 2.0 / 27.0 * t27473 + t30960 / 9.0 + t30964 / 18.0 + t30968 / 27.0 - t30973 / 3.0 - 2.0 / 9.0 * t30976 - t30980 / 18.0 - t24628 - t30984 / 9.0 - 2.0 / 9.0 * t30988 - t30993 / 8.0 - t27792 / 27.0 - t30998 / 6.0 + t31002 / 9.0 + 2.0 / 27.0 * t31006;
    (t31061, t31063, t31064, t31081)
}
