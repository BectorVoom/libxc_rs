//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 820/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk820<F: Float>(t255: F, t9952: F, t258: F, t9570: F, t9577: F, t676: F, t265: F, t9895: F, t2568: F, t737: F, t762: F, t2486: F) -> (F, F, F, F, F, F, F, F) {
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14127 = t676 * t255;
    let t14163 = t9895 * t265;
    let t14175 = t737 * t2568;
    let t14182 = t737 * t762;
    let t14187 = t2486 * t762;
    (t14080, t14081, t14098, t14127, t14163, t14175, t14182, t14187)
}
