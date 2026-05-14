//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 735/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk735<F: Float>(t2007: F, t38351: F, t38355: F, t2310: F, t36542: F, t7720: F, t8597: F, t674: F, t7715: F, t8601: F, t1997: F, t8607: F, t7696: F, t9222: F, t35589: F, t570: F) -> (F, F, F, F, F, F, F, F) {
    let t38361 = t38351 * t2007;
    let t38363 = t38355 * t2007;
    let t38365 = t36542 * t2310;
    let t38367 = t7720 * t8597;
    let t38370 = t8601 * t7715 * t674;
    let t38371 = t38370 * t1997;
    let t38374 = t8607 * t7715 * t674;
    let t38375 = t38374 * t1997;
    let t38377 = t9222 * t7696;
    let t38381 = t35589 * t570;
    (t38361, t38363, t38365, t38367, t38371, t38375, t38377, t38381)
}
