//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1298/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1298(t22233: f64, t22293: f64, t22296: f64, t18451: f64, t18454: f64, t18457: f64, t22230: f64, t22236: f64, t22262: f64, t22265: f64, t22269: f64, t22273: f64, t22277: f64, t22281: f64, t22284: f64, t22287: f64, t22290: f64, t22304: f64, t22306: f64) -> f64 {
    let t22800 = 0.20659e1_f64 * t22233;
    let t22811 = 0.104195e1_f64 * t22293;
    let t22812 = 0.104195e1_f64 * t22296;
    let t22815 = 0.104195e1_f64 * t18451 - 0.62517e0_f64 * t18454 - 0.20839e0_f64 * t18457 - 0.16068111111111111111e1_f64 * t22230 + t22800 - 0.1549425e1_f64 * t22236 + 0.1549425e1_f64 * t22262 - 0.62517e0_f64 * t22265 + 0.312585e0_f64 * t22269 + 0.937755e0_f64 * t22273 + 0.937755e0_f64 * t22277 + 0.312585e0_f64 * t22281 - 0.62517e0_f64 * t22284 - 0.125034e1_f64 * t22287 - 0.92617777777777777779e0_f64 * t22290 + t22811 + t22812 + 0.6311625e0_f64 * t22304 + 0.3529725e1_f64 * t22306;
    t22815
}
