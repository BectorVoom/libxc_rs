//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 766/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk766<F: Float>(t410: F, t6523: F, t2370: F, t6012: F, t2393: F, t937: F, t6455: F, t394: F, t448: F, t452: F, t1424: F, t7: F) -> (F, F, F, F, F, F, F) {
    let t6569 = t6523 * t410;
    let t6570 = t6012 * t2370;
    let t6579 = t2393 * t937;
    let t6590 = t6455 * t410;
    let t6591 = t6012 * t394;
    let t6634 = t448 * t452;
    let t6658 = t7 * t1424;
    (t6569, t6570, t6579, t6590, t6591, t6634, t6658)
}
