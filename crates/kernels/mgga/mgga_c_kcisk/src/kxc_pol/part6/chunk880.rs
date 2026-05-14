//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 880/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk880<F: Float>(t13666: F, t13672: F, t26198: F, t30306: F, t30353: F, t30355: F, t30357: F, t30360: F, t30363: F, t30366: F, t30369: F, t30372: F, t30375: F, t30377: F, t30350: F, t1254: F) -> (F,) {
    let t30379 = -0.29896666666666666667e0 * t30306 + 0.32862666666666666666e0 * t26198 + 0.142419375e1 * t30353 - t13666 - 0.28483875e1 * t30355 + 0.46074375e0 * t30357 + 0.98587999999999999998e0 * t30360 - 0.16431333333333333333e0 * t30363 - 0.73028148148148148146e-1 * t30366 - 0.16431333333333333333e0 * t30369 + 0.32862666666666666666e0 * t30372 - 0.98587999999999999998e0 * t30375 + 0.3071625e0 * t30377 - t13672;
    let t30380 = t30350 + t30379;
    let t30381 = t30380 * t1254;
    (t30381,)
}
