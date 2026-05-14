//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 532/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk532<F: Float>(t7824: F, t7825: F, t446: F, t1647: F, t432: F, t1564: F, t1656: F, t6: F, t1602: F, t66: F) -> (F, F, F, F, F, F, F) {
    let t7826 = t7824 * t7825;
    let t7827 = t446 * t7826;
    let t7829 = t1647 * t432;
    let t7830 = t1564 * t7829;
    let t7831 = t446 * t7830;
    let t7833 = t1656 * t6;
    let t7837 = t1602 * t66;
    (t7826, t7827, t7829, t7830, t7831, t7833, t7837)
}
