//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 618/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk618<F: Float>(t3426: F, t83: F, t1501: F, t1510: F, t1555: F, t1627: F, t3382: F, t3421: F, t3422: F, t3423: F, t3424: F, t3425: F) -> (F, F) {
    let t3427 = t83 * t3426;
    let t3428 = -t3421 + t3422 - t3423 - t3424 - t3425 + t3427 + t3382 + t1627 - t1501 - t1510 - t1555;
    (t3427, t3428)
}
