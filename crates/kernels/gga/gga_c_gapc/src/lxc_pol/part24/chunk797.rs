//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 797/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk797<F: Float>(t9430: F, t9433: F, t9436: F, t9440: F, t9442: F, t9445: F, t9447: F, t9449: F, t9451: F, t9455: F, t9457: F, t9461: F, t9464: F, t9468: F, t9474: F, t9478: F, t9481: F, t9483: F, t9486: F, t9488: F, t9491: F, t9494: F, t9499: F, t9502: F, t9505: F, t9509: F) -> (F, F) {
    let t10827 = -0.55603792169291016668e-2 * t9430 + 0.24326659074064819792e-2 * t9433 - 0.84540905957968605064e-6 * t9436 - 0.28960308421505737848e-5 * t9440 + 0.34752370105806885418e-3 * t9442 + 0.1374296967252737644e-5 * t9445 - 0.4637672555408563478e-4 * t9447 + 0.33816362383187442026e-4 * t9449 - 0.67632724766374884052e-4 * t9451 + 0.2748593934505475288e-6 * t9455 - 0.36652500116630512966e-6 * t9457 - 0.91551759647971344971e-6 * t9461 - 0.2471588561924985691e-3 * t9464;
    let t10842 = -0.2471588561924985691e-3 * t9468 - 0.82386285397499523032e-5 * t9474 + 0.6746961805555555556e-5 * t9478 - 0.4637672555408563478e-4 * t9481 - 0.21642471925239962898e-3 * t9483 - 0.11254699860307667372e-6 * t9486 + 0.55603792169291016668e-2 * t9488 - 0.20240885416666666668e-4 * t9491 - 0.20240885416666666668e-4 * t9494 - 0.22202903123154399017e-4 * t9499 + 0.11272120794395814009e-6 * t9502 - 0.20041830772435757309e-6 * t9505 + 0.55603792169291016668e-2 * t9509;
    (t10827, t10842)
}
