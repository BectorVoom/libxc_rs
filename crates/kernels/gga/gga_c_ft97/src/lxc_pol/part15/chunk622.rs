//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 622/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk622<F: Float>(t255: F, t9952: F, t258: F, t9570: F, t9577: F, t1162: F, t2399: F, t89: F, t676: F, t1160: F, t2492: F, t265: F, t9895: F) -> (F, F, F, F, F, F, F) {
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14098 = t258 * t9577;
    let t14114 = t89 * t2399 * t1162;
    let t14127 = t676 * t255;
    let t14159 = t2492 * t1160;
    let t14163 = t9895 * t265;
    (t14080, t14081, t14098, t14114, t14127, t14159, t14163)
}
