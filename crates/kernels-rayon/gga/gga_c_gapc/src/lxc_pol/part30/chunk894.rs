//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 894/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk894(t9430: f64, t9433: f64, t9436: f64, t9440: f64, t9442: f64, t9445: f64, t9447: f64, t9449: f64, t9451: f64, t9455: f64, t9457: f64, t9461: f64, t9464: f64) -> f64 {
    let t10827 = -0.55603792169291016668e-2_f64 * t9430 + 0.24326659074064819792e-2_f64 * t9433 - 0.84540905957968605064e-6_f64 * t9436 - 0.28960308421505737848e-5_f64 * t9440 + 0.34752370105806885418e-3_f64 * t9442 + 0.1374296967252737644e-5_f64 * t9445 - 0.4637672555408563478e-4_f64 * t9447 + 0.33816362383187442026e-4_f64 * t9449 - 0.67632724766374884052e-4_f64 * t9451 + 0.2748593934505475288e-6_f64 * t9455 - 0.36652500116630512966e-6_f64 * t9457 - 0.91551759647971344971e-6_f64 * t9461 - 0.2471588561924985691e-3_f64 * t9464;
    t10827
}
