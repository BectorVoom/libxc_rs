//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1261/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1261<F: Float>(t11228: F, t25756: F, t35506: F, t35510: F, t35512: F, t35515: F, t35519: F, t35521: F, t35524: F, t35527: F, t35531: F, t35533: F, t35536: F, t35539: F, t35543: F) -> F {
    let t35545 = t11228 * t25756;
    let t35547 = F::new(0.12653481940368541265e-5) * t35506 + F::new(0.7381197798548315738e-6) * t35510 - F::new(0.86898242813537603824e-4) * t35512 + F::new(0.5431140175846100239e-5) * t35515 - F::new(0.5431140175846100239e-5) * t35519 + F::new(0.59742541934307102628e-4) * t35521 - F::new(0.5431140175846100239e-5) * t35524 - F::new(0.27155700879230501195e-5) * t35527 - F::new(0.3218855744218122075e-6) * t35531 - F::new(0.10010310157269334868e-3) * t35533 + F::new(0.27155700879230501195e-5) * t35536 + F::new(0.3218855744218122075e-6) * t35539 + F::new(0.70412469404771420391e-7) * t35543 - F::new(0.2530696388073708253e-5) * t35545;
    t35547
}
