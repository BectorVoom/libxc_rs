//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 799/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk799<F: Float>(t3281: F, t951: F, t1068: F, t3628: F, t1075: F, t1008: F, t8907: F, t1018: F, t2999: F, t89: F, t3000: F, t998: F, t1045: F, t9132: F, t9114: F, t1030: F) -> (F, F, F, F, F, F, F, F, F) {
    let t47926 = t3281 * t951;
    let t48117 = t3628 * t1068;
    let t48442 = t3628 * t1075;
    let t48636 = t8907 * t1008;
    let t49266 = t89 * t2999 * t1018;
    let t49337 = t89 * t3000 * t998;
    let t49622 = t9132 * t1045;
    let t49634 = t9114 * t1045;
    let t49661 = t3281 * t1030;
    (t47926, t48117, t48442, t48636, t49266, t49337, t49622, t49634, t49661)
}
