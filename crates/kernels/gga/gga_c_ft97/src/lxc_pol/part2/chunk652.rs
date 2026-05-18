//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 652/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk652<F: Float>(t10: F, t144: F, t3050: F, t1984: F, t378: F, t2214: F, t8392: F, t2225: F, t582: F, t597: F, t1882: F, t2159: F) -> (F, F, F, F, F, F, F) {
    let t9071 = t10 * t3050 * t144;
    let t9072 = F::new(14.0) / F::new(81.0) * t9071;
    let t9073 = t378 * t1984;
    let t9090 = t8392 * t2214;
    let t9097 = t8392 * t2225;
    let t9099 = t582 * t597;
    let t9106 = t1882 * t2159;
    (t9071, t9072, t9073, t9090, t9097, t9099, t9106)
}
