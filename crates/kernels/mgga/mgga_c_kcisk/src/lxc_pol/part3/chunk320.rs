//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 320/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk320<F: Float>(t529: F, t547: F, t524: F, t1216: F, t41: F, t1287: F, t382: F, t525: F, t526: F, t79: F, t534: F) -> (F, F, F, F, F, F) {
    let t530 = t529 < -F::new(0.66725e-1);
    let t1555 = t547 * t547;
    let t1556 = F::new(1.0) / t1555;
    let t1557 = t524 * t1556;
    let t1558 = t1216 * t41;
    let t1566 = piecewise3::<f64>(t530, F::new(0.0), F::new(10.0) / F::new(9.0) * t525 * t1558 * t382 - F::new(10.0) / F::new(27.0) * t525 * t526 * t1287);
    let t1567 = t79 * t1566;
    let t1568 = t1567 * t534;
    (t1555, t1556, t1557, t1558, t1567, t1568)
}
