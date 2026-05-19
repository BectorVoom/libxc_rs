//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 887/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk887<F: Float>(t34960: F, t356: F, t638: F, t639: F, t8849: F, t34750: F, t34755: F, t577: F, t2392: F, t866: F, t262: F, t8620: F) -> (F, F, F, F, F, F) {
    let t39364 = F::cast_from(0.2927036860455597649e0_f64) * t34960;
    let t39367 = t638 * t639 * t8849 * t356;
    let t39370 = t34755 * t577 * t34750;
    let t39372 = t2392 * t866;
    let t39373 = t262 * t39372;
    let t39374 = t8620 * t39373;
    (t39364, t39367, t39370, t39372, t39373, t39374)
}
