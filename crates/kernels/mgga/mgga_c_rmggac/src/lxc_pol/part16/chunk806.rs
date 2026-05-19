//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 806/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk806<F: Float>(t34960: F, t34750: F, t34755: F, t577: F, t2339: F, t638: F, t7184: F, t1965: F, t9085: F, t1969: F, t2305: F, t35654: F) -> (F, F, F, F, F) {
    let t39364 = F::cast_from(0.2927036860455597649e0_f64) * t34960;
    let t39370 = t34755 * t577 * t34750;
    let t39388 = t638 * t7184 * t2339;
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39405 = t35654 * t2305;
    (t39364, t39370, t39388, t39393, t39405)
}
