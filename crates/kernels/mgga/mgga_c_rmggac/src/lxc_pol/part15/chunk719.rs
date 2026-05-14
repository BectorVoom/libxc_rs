//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 719/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk719<F: Float>(t38822: F, t2019: F, t38815: F, t640: F, t7764: F, t2338: F, t7352: F, t1664: F, t2010: F, t7755: F, t7556: F, t7553: F, t7555: F, t31: F, t574: F, t34795: F, t529: F) -> (F, F, F, F, F, F, F, F, F) {
    let t38823 = 0.10248087766267884742e-3 * t38822;
    let t38826 = t2019 * t7764 * t640 * t38815;
    let t38833 = t2019 * t7764 * t2338 * t7352;
    let t38835 = t1664 * t7352;
    let t38837 = t2010 * t7755 * t38835;
    let t38838 = 0.72042316457491791906e-3 * t38837;
    let t38839 = t2338 * t7556;
    let t38841 = t7553 * t7555 * t38839;
    let t38843 = t574 * t31;
    let t38844 = t640 * t38843;
    let t38846 = t7553 * t7555 * t38844;
    let t38848 = t34795 * t529;
    (t38823, t38826, t38833, t38835, t38838, t38841, t38843, t38846, t38848)
}
