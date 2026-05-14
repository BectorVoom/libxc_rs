//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1199/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1199<F: Float>(t2411: F, t2888: F, t2099: F, t3235: F, t8419: F, t8410: F, t8414: F, t23213: F, t3206: F, t8255: F, t1220: F, t6433: F, t154: F, t3026: F, t385: F, t6446: F) -> (F, F, F, F, F, F, F) {
    let t23278 = t2888 * t2411;
    let t23286 = t3235 * t2099 * t8419;
    let t23296 = t3235 * t2099 * t8410;
    let t23299 = t3235 * t2099 * t8414;
    let t23311 = t3206 * t23213 * t8255;
    let t23313 = t1220 * t6433;
    let t23317 = t385 * t154 * t6446 * t3026;
    (t23278, t23286, t23296, t23299, t23311, t23313, t23317)
}
