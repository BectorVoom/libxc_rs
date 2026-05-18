//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 806/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk806<F: Float>(t39338: F, t2338: F, t7323: F, t7324: F, t1327: F, t574: F, t640: F, t34750: F, t34755: F, t577: F, t2339: F, t638: F, t7184: F) -> (F, F, F, F, F) {
    let t39339 = F::new(0.30487649791575028314e-3) * t39338;
    let t39341 = t7323 * t2338 * t7324;
    let t39345 = t7323 * t640 * t574 * t1327;
    let t39370 = t34755 * t577 * t34750;
    let t39388 = t638 * t7184 * t2339;
    (t39339, t39341, t39345, t39370, t39388)
}
