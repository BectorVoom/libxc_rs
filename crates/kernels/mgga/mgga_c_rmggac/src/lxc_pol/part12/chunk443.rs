//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 443/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk443<F: Float>(t504: F, t837: F, t4035: F, t529: F, t1368: F, t866: F, t551: F, t874: F, t876: F, t559: F, t833: F, t124: F, t235: F, t839: F, t571: F, t794: F) -> (F, F, F, F, F, F, F, F) {
    let t5019 = t504 * t837;
    let t5026 = t4035 * t529;
    let t5029 = t1368 * t866;
    let t5032 = t874 * t551;
    let t5033 = t5032 * t876;
    let t5041 = t559 * t833;
    let t5048 = t235 * t124;
    let t5049 = t559 * t839;
    let t5052 = t571 * t794;
    (t5019, t5026, t5029, t5033, t5041, t5048, t5049, t5052)
}
