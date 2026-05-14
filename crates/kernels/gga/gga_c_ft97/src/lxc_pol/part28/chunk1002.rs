//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1002/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1002<F: Float>(t18: F, t3281: F, t32962: F, t9073: F, t23657: F, t27152: F, t32924: F, t9432: F, t1882: F, t34936: F, t34918: F, t358: F, t1969: F, t363: F, t446: F, t34924: F) -> (F, F, F, F, F) {
    let t148567 = t3281 * t9073 * t32962 * t18;
    let t148571 = t23657 * t9432 * t32924 * t27152;
    let t148573 = t1882 * t34936;
    let t148575 = t34918 * t358;
    let t148578 = t446 * t1969 * t148575 * t363;
    let t148580 = t1882 * t34924;
    (t148567, t148571, t148573, t148578, t148580)
}
