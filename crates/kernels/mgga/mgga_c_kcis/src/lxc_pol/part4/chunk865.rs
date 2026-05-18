//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 865/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk865<F: Float>(t1363: F, t1924: F, t1466: F, t1981: F, t1490: F, t1464: F, t2050: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t5742 = t1924 * t1363;
    let t5747 = t1981 * t1466;
    let t5748 = t5747 * sigma2;
    let t5749 = t5748 * t1490;
    let t5750 = t1464 * t5749;
    let t5752 = t2050 * sigma2;
    (t5742, t5747, t5748, t5749, t5750, t5752)
}
