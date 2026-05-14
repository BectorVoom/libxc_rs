//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1162/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1162<F: Float>(t22233: F, t18451: F, t18454: F, t18457: F, t22230: F, t22236: F, t22262: F, t22265: F, t22269: F, t22273: F, t22277: F, t22281: F, t22284: F, t22287: F, t22290: F, t22293: F, t22296: F, t22304: F, t22306: F) -> (F,) {
    let t22336 = 0.11958666666666666667e1 * t22233;
    let t22351 = 0.82156666666666666666e0 * t18451 - 0.49293999999999999999e0 * t18454 - 0.16431333333333333333e0 * t18457 - 0.93011851851851851854e0 * t22230 + t22336 - 0.89690000000000000001e0 * t22236 + 0.8969e0 * t22262 - 0.49293999999999999999e0 * t22265 + 0.24647e0 * t22269 + 0.73941e0 * t22273 + 0.73941e0 * t22277 + 0.24647e0 * t22281 - 0.49293999999999999999e0 * t22284 - 0.98587999999999999999e0 * t22287 - 0.73028148148148148147e0 * t22290 + 0.82156666666666666665e0 * t22293 + 0.82156666666666666665e0 * t22296 + 0.3071625e0 * t22304 + 0.1898925e1 * t22306;
    (t22351,)
}
