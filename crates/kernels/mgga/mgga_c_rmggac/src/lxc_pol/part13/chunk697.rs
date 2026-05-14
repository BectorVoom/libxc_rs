//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 697/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk697<F: Float>(t1982: F, t7428: F, t7542: F, t321: F, t7817: F, t1550: F, t333: F, t903: F, t338: F, t830: F, t352: F, t739: F, t2157: F, t4685: F, t131: F, t1338: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t35580 = t7542 * t7428 * t1982;
    let t35583 = t7817 * t321;
    let t35584 = t1550 * t35583;
    let t35586 = t7817 * t333;
    let t35587 = t903 * t35586;
    let t35589 = t338 * t830;
    let t35590 = t35589 * t352;
    let t35591 = t739 * t35590;
    let t35594 = t4685 * t2157;
    let t35604 = t131 * t1338;
    (t35580, t35583, t35584, t35586, t35587, t35589, t35590, t35591, t35594, t35604)
}
