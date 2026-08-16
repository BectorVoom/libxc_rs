//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 642/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk642<F: Float>(t2004: F, t8571: F, t2007: F, t1965: F, t2410: F, t1969: F) -> (F, F, F, F) {
    let t8572 = t8571 * t2004;
    let t8574 = t8571 * t2007;
    let t8576 = t2410 * t1965;
    let t8577 = t8576 * t1969;
    (t8572, t8574, t8576, t8577)
}
