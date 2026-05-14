//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1072/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1072<F: Float>(t61159: F, t61208: F, t3404: F, t554: F, t538: F, t1013: F, t2059: F, t2071: F, t1060: F, t9132: F, t1045: F, t526: F, t12664: F, t582: F, t1985: F, t2179: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61209 = t61159 + t61208;
    let t61637 = t554 * t3404;
    let t61641 = t3404 * t538;
    let t61777 = t1013 * t2059;
    let t61786 = t1013 * t2071;
    let t63052 = t9132 * t1060;
    let t63180 = t526 * t1045;
    let t63304 = t582 * t12664;
    let t63755 = t1985 * t2179;
    (t61209, t61637, t61641, t61777, t61786, t63052, t63180, t63304, t63755)
}
