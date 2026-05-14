//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 713/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk713<F: Float>(t38354: F, t674: F, t7715: F, t8601: F, t8607: F, t35589: F, t570: F, t739: F, t1609: F, t1986: F, t7244: F, t8447: F, t205: F, t24985: F, t3350: F, t671: F) -> (F, F, F, F, F, F, F, F) {
    let t38355 = t38354 * t674;
    let t38370 = t8601 * t7715 * t674;
    let t38374 = t8607 * t7715 * t674;
    let t38381 = t35589 * t570;
    let t38382 = t739 * t38381;
    let t38397 = t1986 * t1609;
    let t38414 = t7244 * t8447;
    let t38415 = 0.19863479950205658386e-4 * t38414;
    let t38454 = t671 * t24985 * t205 * t3350;
    (t38355, t38370, t38374, t38381, t38382, t38397, t38415, t38454)
}
