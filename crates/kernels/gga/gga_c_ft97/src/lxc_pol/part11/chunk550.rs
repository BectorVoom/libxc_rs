//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 550/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk550<F: Float>(t447: F, t7789: F, t446: F, t1642: F, t369: F, t1643: F, t432: F, t7748: F, t7754: F, t7758: F, t7768: F, t7771: F, t7775: F, t7778: F, t7782: F, t7786: F) -> (F, F, F, F, F, F, F) {
    let t7790 = t447 * t7789;
    let t7791 = t446 * t7790;
    let t7793 = t1642 * t369;
    let t7794 = t1643 * t432;
    let t7795 = t7793 * t7794;
    let t7796 = t446 * t7795;
    let t7798 = -t7748 / F::new(18.0) - t7754 + t7758 - F::new(5.0) / F::new(81.0) * t7768 - t7771 / F::new(3.0) - F::new(2.0) / F::new(27.0) * t7775 + t7778 / F::new(18.0) + t7782 / F::new(27.0) - t7786 / F::new(3.0) + t7791 / F::new(3.0) + t7796 / F::new(9.0);
    (t7790, t7791, t7793, t7794, t7795, t7796, t7798)
}
