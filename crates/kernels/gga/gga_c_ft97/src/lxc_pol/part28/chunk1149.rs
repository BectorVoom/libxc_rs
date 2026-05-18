//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1149/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1149<F: Float>(t1882: F, t34936: F, t34918: F, t358: F, t1969: F, t363: F, t446: F, t34924: F, t34844: F, t376: F, t5890: F, t148517: F, t2112: F, t28: F) -> (F, F, F, F, F) {
    let t148573 = t1882 * t34936;
    let t148575 = t34918 * t358;
    let t148578 = t446 * t1969 * t148575 * t363;
    let t148580 = t1882 * t34924;
    let t148583 = t5890 * t376 * t34844;
    let t148587 = t5890 * t28 * t2112 * t148517;
    (t148573, t148578, t148580, t148583, t148587)
}
