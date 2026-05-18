//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1023/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1023<F: Float>(t8529: F, t10285: F, t10286: F, t37108: F, t42424: F, t42425: F, t42426: F, t42427: F, t42428: F, t7383: F, t7391: F, t9333: F) -> (F, F) {
    let t42429 = F::new(0.5454932330849068346e-1) * t8529;
    let t42431 = F::new(0.31931311204970156171e0) * t7383 - t42424 + t42425 + t42426 + t42427 + t42428 - t42429 + t10285 - t10286 - t37108 + F::new(0.17347588262831798123e-3) * t7391;
    let t42434 = F::new(0.11974241701863808564e0) * t9333;
    (t42431, t42434)
}
