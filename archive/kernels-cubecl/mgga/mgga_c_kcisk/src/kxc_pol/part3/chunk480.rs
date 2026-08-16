//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 480/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk480<F: Float>(t1340: F, t3760: F, t3759: F, t1299: F, t470: F) -> (F, F, F) {
    let t3761 = t1340 * t3760;
    let t3762 = t3759 * t3761;
    let t3764 = t1299 * t470;
    (t3761, t3762, t3764)
}
