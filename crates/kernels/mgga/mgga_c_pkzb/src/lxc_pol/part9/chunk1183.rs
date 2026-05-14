//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1183/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1183<F: Float>(t22233: F, t22293: F, t22296: F, t18451: F, t18454: F, t18457: F, t22230: F, t22236: F, t22262: F, t22265: F, t22269: F, t22273: F, t22277: F, t22281: F, t22284: F, t22287: F, t22290: F, t22304: F, t22306: F) -> (F,) {
    let t22800 = 0.20659e1 * t22233;
    let t22811 = 0.104195e1 * t22293;
    let t22812 = 0.104195e1 * t22296;
    let t22815 = 0.104195e1 * t18451 - 0.62517e0 * t18454 - 0.20839e0 * t18457 - 0.16068111111111111111e1 * t22230 + t22800 - 0.1549425e1 * t22236 + 0.1549425e1 * t22262 - 0.62517e0 * t22265 + 0.312585e0 * t22269 + 0.937755e0 * t22273 + 0.937755e0 * t22277 + 0.312585e0 * t22281 - 0.62517e0 * t22284 - 0.125034e1 * t22287 - 0.92617777777777777779e0 * t22290 + t22811 + t22812 + 0.6311625e0 * t22304 + 0.3529725e1 * t22306;
    (t22815,)
}
