//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 849/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk849<F: Float>(t38843: F, t640: F, t7553: F, t7555: F, t34795: F, t529: F, t2010: F, t34797: F, t2415: F, t35220: F, t7349: F, t1411: F, t7754: F) -> (F, F, F, F) {
    let t38844 = t640 * t38843;
    let t38846 = t7553 * t7555 * t38844;
    let t38848 = t34795 * t529;
    let t38850 = t2010 * t38848 * t34797;
    let t38853 = t7349 * t2415 * t35220;
    let t38854 = F::new(0.10248087766267884742e-3) * t38853;
    let t38855 = t7754 * t1411;
    (t38846, t38850, t38854, t38855)
}
