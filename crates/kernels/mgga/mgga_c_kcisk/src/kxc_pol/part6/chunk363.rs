//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 363/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk363<F: Float>(t529: F, t2110: F, t41: F, t2153: F, t382: F, t525: F, t526: F, t79: F, t534: F) -> (F, F, F) {
    let t530 = t529 < -F::new(0.66725e-1);
    let t2308 = t2110 * t41;
    let t2316 = piecewise3::<f64>(t530, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t2308 * t382 - F::new(10.0) / F::new(27.0) * t525 * t526 * t2153);
    let t2317 = t79 * t2316;
    let t2318 = t2317 * t534;
    (t2308, t2317, t2318)
}
