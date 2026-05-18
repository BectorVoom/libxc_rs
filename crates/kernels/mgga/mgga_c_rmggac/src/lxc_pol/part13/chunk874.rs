//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 874/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk874<F: Float>(t34761: F, t9153: F, t16502: F, t8516: F, t2318: F, t34976: F, t7455: F, t3369: F, t34975: F, t559: F, t35039: F, t7461: F) -> (F, F, F, F) {
    let t39435 = t34761 * t9153;
    let t39437 = t8516 * t16502;
    let t39440 = t39437 * t34976 * t2318 * t7455;
    let t39445 = t34975 * t3369 * t559 * t7455;
    let t39449 = t34975 * t35039 * t2318 * t7461;
    (t39435, t39440, t39445, t39449)
}
