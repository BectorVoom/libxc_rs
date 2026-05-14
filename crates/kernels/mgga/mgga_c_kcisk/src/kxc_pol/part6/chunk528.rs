//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 528/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk528<F: Float>(t3598: F, t7757: F, t3572: F, t5668: F, t7738: F, t7742: F, t7746: F) -> (F, F) {
    let t7758 = t3598 * t7757;
    let t7764 = t3572 + 2.0 / 9.0 * t5668 - 2.0 / 9.0 * t7738 + 2.0 / 3.0 * t7742 - t7746 / 3.0;
    (t7758, t7764)
}
