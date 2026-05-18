//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 991/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk991<F: Float>(t1248: F, t1249: F, t30238: F, t30290: F, t4065: F, t30298: F, t1242: F, t30339: F, t13666: F, t13672: F, t26198: F, t30306: F, t30353: F, t30355: F, t30357: F, t30360: F, t30363: F, t30366: F) -> (F, F, F, F, F) {
    let t30369 = t1248 * t1249 * t30238;
    let t30372 = t1248 * t4065 * t30290;
    let t30375 = t1248 * t1249 * t30298;
    let t30377 = t1242 * t30339;
    let t30379 = -F::new(0.29896666666666666667e0) * t30306 + F::new(0.32862666666666666666e0) * t26198 + F::new(0.142419375e1) * t30353 - t13666 - F::new(0.28483875e1) * t30355 + F::new(0.46074375e0) * t30357 + F::new(0.98587999999999999998e0) * t30360 - F::new(0.16431333333333333333e0) * t30363 - F::new(0.73028148148148148146e-1) * t30366 - F::new(0.16431333333333333333e0) * t30369 + F::new(0.32862666666666666666e0) * t30372 - F::new(0.98587999999999999998e0) * t30375 + F::new(0.3071625e0) * t30377 - t13672;
    (t30369, t30372, t30375, t30377, t30379)
}
