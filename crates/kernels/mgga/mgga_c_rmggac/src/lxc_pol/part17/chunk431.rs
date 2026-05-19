//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 431/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk431<F: Float>(t4248: F, t4333: F, t156: F, t155: F, t1132: F, t385: F, t1045: F, t980: F, t180: F, t243: F, t483: F, t426: F) -> (F, F, F, F) {
    let t4334 = t4248 + t4333;
    let t4335 = t156 * t4334;
    let t4336 = t155 * t4335;
    let t4338 = F::new(12.0) * t385 * t1132;
    let t4342 = t1045 * t980;
    let t4349 = t243 * t483 * t180;
    let t4351 = F::cast_from(0.56968947174242584612e-3_f64) * t426 * t4349;
    (t4336, t4338, t4342, t4351)
}
