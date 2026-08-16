//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2184/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2184<F: Float>(t12283: F, t19962: F, t19882: F, t19996: F, t3866: F, t40018: F, t6371: F, t12189: F, t6375: F, t40138: F, t6396: F, t19951: F) -> (F, F, F, F, F, F, F) {
    let t56933 = t12283 * t19962;
    let t56935 = t12283 * t19882;
    let t56937 = t3866 * t19996;
    let t56946 = t40018 * t6371;
    let t56953 = t12189 * t6375;
    let t56959 = t40138 * t6396;
    let t56961 = t12283 * t19951;
    (t56933, t56935, t56937, t56946, t56953, t56959, t56961)
}
