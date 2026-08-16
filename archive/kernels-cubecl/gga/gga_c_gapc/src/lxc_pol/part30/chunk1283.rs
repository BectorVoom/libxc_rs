//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1283/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1283<F: Float>(t35506: F, t35510: F, t35512: F, t35515: F, t35519: F, t35521: F, t35524: F, t35527: F, t35531: F, t35533: F, t35536: F, t35539: F, t35543: F, t35545: F) -> F {
    let t37414 = F::cast_from(0.50613927761474165061e-5_f64) * t35506 + F::cast_from(0.29524791194193262952e-5_f64) * t35510 - F::cast_from(0.3475929712541504153e-3_f64) * t35512 + F::cast_from(0.21724560703384400956e-4_f64) * t35515 - F::cast_from(0.21724560703384400956e-4_f64) * t35519 + F::cast_from(0.23897016773722841052e-3_f64) * t35521 - F::cast_from(0.21724560703384400956e-4_f64) * t35524 - F::cast_from(0.10862280351692200478e-4_f64) * t35527 - F::cast_from(0.128754229768724883e-5_f64) * t35531 - F::cast_from(0.4004124062907733947e-3_f64) * t35533 + F::cast_from(0.10862280351692200478e-4_f64) * t35536 + F::cast_from(0.128754229768724883e-5_f64) * t35539 + F::cast_from(0.28164987761908568157e-6_f64) * t35543 - F::cast_from(0.10122785552294833012e-4_f64) * t35545;
    t37414
}
