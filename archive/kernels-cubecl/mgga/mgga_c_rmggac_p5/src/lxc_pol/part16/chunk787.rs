//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 787/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk787<F: Float>(t1341: F, t535: F, t638: F, t7310: F, t5542: F, t8601: F, t674: F, t8607: F, t7715: F, t35589: F, t570: F, t739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38326 = t638 * t7310 * t535 * t1341;
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38370 = t8601 * t7715 * t674;
    let t38374 = t8607 * t7715 * t674;
    let t38381 = t35589 * t570;
    let t38382 = t739 * t38381;
    (t38326, t38350, t38351, t38354, t38355, t38370, t38374, t38381, t38382)
}
