//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 797/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk797<F: Float>(t2010: F, t38855: F, t7756: F, t34715: F, t8465: F, t35215: F, t35623: F, t7349: F, t7760: F, t8342: F, t2415: F, t35210: F) -> (F, F, F, F, F, F) {
    let t38857 = t2010 * t38855 * t7756;
    let t38858 = F::new(0.72042316457491791906e-3) * t38857;
    let t38860 = t2010 * t8465 * t34715;
    let t38861 = F::new(0.72042316457491791906e-3) * t38860;
    let t38863 = t2010 * t8465 * t35215;
    let t38864 = F::new(0.72042316457491791906e-3) * t38863;
    let t38866 = t2010 * t8465 * t35623;
    let t38869 = t7349 * t8342 * t7760;
    let t38870 = F::new(0.10248087766267884742e-3) * t38869;
    let t38872 = t7349 * t2415 * t35210;
    (t38858, t38861, t38864, t38866, t38870, t38872)
}
