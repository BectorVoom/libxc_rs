//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 815/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk815<F: Float>(t2131: F, t5026: F, t7244: F, t9171: F, t1540: F, t2144: F, t36734: F, t8443: F, t8437: F, t36292: F, t5888: F, t739: F) -> (F, F, F, F, F, F) {
    let t39923 = t5026 * t2131;
    let t39926 = t7244 * t9171;
    let t39927 = F::new(0.19863479950205658386e-4) * t39926;
    let t39953 = t1540 * t2144;
    let t39970 = t36734 * t8443;
    let t39971 = F::new(0.19863479950205658386e-4) * t39970;
    let t39977 = t7244 * t8437;
    let t39978 = F::new(0.19863479950205658386e-4) * t39977;
    let t39997 = t739 * t36292 * t5888;
    (t39923, t39927, t39953, t39971, t39978, t39997)
}
