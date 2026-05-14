//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 943/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk943<F: Float>(t12929: F, t12931: F, t12933: F, t12948: F, t12975: F, t19100: F, t19103: F, t19105: F, t19106: F, t19111: F, t19116: F, t19121: F, t19125: F, t19129: F, t19134: F, t19138: F, t19142: F) -> (F,) {
    let t19144 = -t12975 - 8.0 / 27.0 * t12929 + 2.0 / 27.0 * t12933 - 2.0 / 9.0 * t12948 + t12931 / 9.0 - 4.0 / 27.0 * t19100 + t19103 - t19105 + 22.0 / 9.0 * t19106 - 10.0 / 27.0 * t19111 + 4.0 / 3.0 * t19116 - 8.0 / 9.0 * t19121 - 2.0 / 9.0 * t19125 - 2.0 * t19129 + 8.0 / 3.0 * t19134 + 2.0 / 3.0 * t19138 - 2.0 / 3.0 * t19142;
    (t19144,)
}
