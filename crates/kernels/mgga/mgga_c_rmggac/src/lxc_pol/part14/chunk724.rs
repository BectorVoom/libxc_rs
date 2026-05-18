//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 724/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk724<F: Float>(t558: F, t866: F, t7186: F, t7294: F, t7299: F, t7313: F, t7326: F, t7336: F, t7346: F, t7355: F, t7387: F, t7492: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31125 = t558 * t866;
    let t34521 = F::new(0.44715219694310041527e-2) * t7186;
    let t34544 = F::new(0.24390119833260022651e-2) * t7294;
    let t34545 = F::new(0.5854811038705731867e-3) * t7299;
    let t34548 = F::new(0.91462949374725084942e-3) * t7313;
    let t34551 = F::new(0.10260057759007034251e-5) * t7326;
    let t34554 = F::new(0.45731474687362542471e-3) * t7336;
    let t34557 = F::new(0.45731474687362542471e-3) * t7346;
    let t34558 = F::new(0.13010691197123848594e-3) * t7355;
    let t34567 = F::new(0.45731474687362542471e-3) * t7387;
    let t34592 = F::new(0.91462949374725084942e-3) * t7492;
    (t31125, t34521, t34544, t34545, t34548, t34551, t34554, t34557, t34558, t34567, t34592)
}
