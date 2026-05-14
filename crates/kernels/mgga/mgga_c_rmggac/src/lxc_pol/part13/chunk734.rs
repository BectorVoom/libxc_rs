//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 734/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk734<F: Float>(t674: F, t7715: F, t8601: F, t1997: F, t8607: F, t7696: F, t9222: F, t35589: F, t570: F, t739: F, t7255: F, t9171: F, t7463: F, t8577: F, t7469: F, t7484: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38370 = t8601 * t7715 * t674;
    let t38371 = t38370 * t1997;
    let t38374 = t8607 * t7715 * t674;
    let t38375 = t38374 * t1997;
    let t38377 = t9222 * t7696;
    let t38381 = t35589 * t570;
    let t38382 = t739 * t38381;
    let t38387 = t7255 * t9171;
    let t38389 = t8577 * t7463;
    let t38391 = t8577 * t7469;
    let t38393 = t8577 * t7484;
    (t38371, t38375, t38377, t38381, t38382, t38387, t38389, t38391, t38393)
}
