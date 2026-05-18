//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 942/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk942<F: Float>(t9430: F, t9433: F, t9436: F, t9440: F, t9442: F, t9445: F, t9447: F, t9449: F, t9451: F, t9455: F, t9457: F, t9461: F, t9464: F) -> F {
    let t10827 = -F::new(0.55603792169291016668e-2) * t9430 + F::new(0.24326659074064819792e-2) * t9433 - F::new(0.84540905957968605064e-6) * t9436 - F::new(0.28960308421505737848e-5) * t9440 + F::new(0.34752370105806885418e-3) * t9442 + F::new(0.1374296967252737644e-5) * t9445 - F::new(0.4637672555408563478e-4) * t9447 + F::new(0.33816362383187442026e-4) * t9449 - F::new(0.67632724766374884052e-4) * t9451 + F::new(0.2748593934505475288e-6) * t9455 - F::new(0.36652500116630512966e-6) * t9457 - F::new(0.91551759647971344971e-6) * t9461 - F::new(0.2471588561924985691e-3) * t9464;
    t10827
}
