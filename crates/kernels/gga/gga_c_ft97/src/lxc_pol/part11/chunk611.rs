//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 611/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk611<F: Float>(t9016: F, t9017: F, t27: F, t89: F, t1984: F, t2075: F, t558: F, t28: F, t143: F, t7763: F, t7765: F, t7761: F, t1964: F, t356: F, t569: F, t7789: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9018 = t9016 * t9017;
    let t9020 = t89 * t27 * t9018;
    let t9022 = t1984 * t558 * t2075;
    let t9024 = t89 * t28 * t9022;
    let t9025 = t143 * t7763;
    let t9026 = t9025 * t7765;
    let t9028 = t89 * t7761 * t9026;
    let t9030 = t1964 * t7765;
    let t9032 = t89 * t356 * t9030;
    let t9034 = t569 * t7789;
    (t9018, t9020, t9022, t9024, t9025, t9026, t9028, t9030, t9032, t9034)
}
