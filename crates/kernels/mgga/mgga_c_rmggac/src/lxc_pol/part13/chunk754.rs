//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 754/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk754<F: Float>(t1664: F, t7352: F, t2010: F, t7755: F, t2338: F, t7556: F, t7553: F, t7555: F, t31: F, t574: F, t640: F, t34795: F, t529: F, t34797: F, t2415: F, t35220: F, t7349: F) -> (F, F, F, F, F, F) {
    let t38835 = t1664 * t7352;
    let t38837 = t2010 * t7755 * t38835;
    let t38839 = t2338 * t7556;
    let t38841 = t7553 * t7555 * t38839;
    let t38843 = t574 * t31;
    let t38844 = t640 * t38843;
    let t38846 = t7553 * t7555 * t38844;
    let t38848 = t34795 * t529;
    let t38850 = t2010 * t38848 * t34797;
    let t38853 = t7349 * t2415 * t35220;
    (t38837, t38841, t38843, t38846, t38850, t38853)
}
