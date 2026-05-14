//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 519/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk519<F: Float>(t332: F, t5478: F, t113: F, t1274: F, t992: F, t6: F, t694: F, t373: F, t929: F, t1095: F, t679: F) -> (F, F, F, F, F, F) {
    let t5479 = t5478 * t332;
    let t5480 = t5479 * t113;
    let t5483 = t1274 * t992;
    let t6032 = t694 * t6;
    let t6426 = t373 * t929;
    let t6757 = t679 * t1095;
    (t5479, t5480, t5483, t6032, t6426, t6757)
}
