//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 356/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk356<F: Float>(t524: F, t1581: F, t2059: F, t1312: F, t2306: F) -> (F, F, F) {
    let t536 = 0.0 < t524;
    let t2321 = t1581 * t2059;
    let t2322 = t1312 * t2321;
    let t2326 = piecewise3(t536, t2306, -t2306);
    (t2321, t2322, t2326)
}
