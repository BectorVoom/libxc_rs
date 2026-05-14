//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 932/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk932<F: Float>(t356: F, t519: F, t85501: F, t89: F, t85469: F, t9054: F, t1974: F, t85451: F, t1555: F, t1964: F, t20714: F, t925: F, t446: F, t9073: F, t20758: F, t2983: F) -> (F, F, F, F, F, F, F) {
    let t86958 = t89 * t356 * t519 * t85501;
    let t86962 = t89 * t356 * t9054 * t85469;
    let t86966 = t89 * t356 * t1974 * t85451;
    let t86970 = t89 * t1555 * t1964 * t85451;
    let t86973 = t925 * t20714;
    let t86975 = t446 * t9073 * t86973;
    let t86977 = t2983 * t20758;
    (t86958, t86962, t86966, t86970, t86973, t86975, t86977)
}
