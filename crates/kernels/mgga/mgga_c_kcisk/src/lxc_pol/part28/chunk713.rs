//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 713/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk713<F: Float>(t604: F, t5031: F, t8814: F, t1310: F, t8616: F) -> (F, F, F) {
    let t659 = 0.0 < t604;
    let t8815 = t5031 * t8814;
    let t8816 = t1310 * t8815;
    let t8820 = piecewise3(t659, t8616, -t8616);
    (t8815, t8816, t8820)
}
