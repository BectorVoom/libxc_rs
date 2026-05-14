//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 797/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk797<F: Float>(t4135: F, t6922: F, t1395: F, t1464: F, t2001: F) -> (F, F, F, F) {
    let t6923 = t4135 * t6922;
    let t6924 = t1395 * t6923;
    let t6925 = t1464 * t6924;
    let t6927 = t2001 * t2001;
    (t6923, t6924, t6925, t6927)
}
