//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 543/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk543<F: Float>(t201: F, t7546: F, t1979: F, t1982: F, t1320: F, t1322: F, t1325: F, t2016: F) -> (F, F, F, F, F) {
    let t7547 = t7546 * t201;
    let t7549 = t7547 * t1979 * t1982;
    let t7550 = 0.42564599893297839398e-5 * t7549;
    let t7551 = t1320 * t1322;
    let t7552 = t7551 * t1325;
    let t7553 = t2016 * t7552;
    (t7547, t7550, t7551, t7552, t7553)
}
