//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 860/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk860<F: Float>(t3862: F, t555: F, t1361: F, t835: F, t1336: F) -> (F, F, F) {
    let t3864 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    (t3864, t3865, t3866)
}
