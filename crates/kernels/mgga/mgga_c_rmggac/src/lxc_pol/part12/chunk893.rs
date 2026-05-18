//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 893/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk893<F: Float>(t3369: F, t34975: F, t559: F, t7455: F, t2318: F, t35039: F, t7461: F, t5016: F, t9000: F, t16043: F, t8812: F, t2320: F, t35146: F) -> (F, F, F, F, F) {
    let t39445 = t34975 * t3369 * t559 * t7455;
    let t39449 = t34975 * t35039 * t2318 * t7461;
    let t39451 = t5016 * t9000;
    let t39452 = F::new(0.15965655602485078085e0) * t39451;
    let t39453 = t16043 * t8812;
    let t39455 = t35146 * t2320;
    (t39445, t39449, t39452, t39453, t39455)
}
