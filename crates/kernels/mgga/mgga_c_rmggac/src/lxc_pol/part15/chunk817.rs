//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 817/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk817<F: Float>(t2318: F, t34975: F, t35039: F, t8420: F, t16504: F, t8425: F, t3369: F, t8430: F, t34976: F, t39866: F, t8435: F, t321: F, t9876: F, t262: F, t7198: F, t1469: F, t40145: F) -> (F, F, F, F, F, F, F, F) {
    let t45403 = t34975 * t35039 * t2318 * t8420;
    let t45407 = t34975 * t16504 * t2318 * t8425;
    let t45411 = t34975 * t3369 * t2318 * t8430;
    let t45415 = t34975 * t34976 * t39866 * t8435;
    let t45418 = t9876 * t321;
    let t45419 = t262 * t45418;
    let t45420 = t7198 * t45419;
    let t45424 = t40145 * t34976 * t2318 * t1469;
    (t45403, t45407, t45411, t45415, t45418, t45419, t45420, t45424)
}
