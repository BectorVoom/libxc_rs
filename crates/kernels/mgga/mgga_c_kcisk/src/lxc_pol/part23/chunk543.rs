//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 543/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk543<F: Float>(t1341: F, t3575: F, t1340: F, t3759: F, t1299: F, t470: F) -> (F, F, F, F) {
    let t3760 = t1341 * t3575;
    let t3761 = t1340 * t3760;
    let t3762 = t3759 * t3761;
    let t3764 = t1299 * t470;
    (t3760, t3761, t3762, t3764)
}
