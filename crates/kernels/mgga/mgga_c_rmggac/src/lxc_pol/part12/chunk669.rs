//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 669/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk669<F: Float>(t7313: F, t7326: F, t7336: F, t7346: F, t7355: F, t7387: F, t7492: F, t7559: F, t7562: F, t7767: F, t2181: F, t7561: F, t2165: F, t638: F, t7184: F, t2169: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t34548 = 0.91462949374725084942e-3 * t7313;
    let t34551 = 0.10260057759007034251e-5 * t7326;
    let t34554 = 0.45731474687362542471e-3 * t7336;
    let t34557 = 0.45731474687362542471e-3 * t7346;
    let t34558 = 0.13010691197123848594e-3 * t7355;
    let t34567 = 0.45731474687362542471e-3 * t7387;
    let t34592 = 0.91462949374725084942e-3 * t7492;
    let t34612 = 0.13010691197123848594e-3 * t7559;
    let t34613 = 0.10000709273223291967e0 * t7562;
    let t34649 = 0.91462949374725084942e-3 * t7767;
    let t34659 = t2181 * t7561;
    let t34662 = t638 * t7184 * t2165;
    let t34665 = t638 * t7184 * t2169;
    (t34548, t34551, t34554, t34557, t34558, t34567, t34592, t34612, t34613, t34649, t34659, t34662, t34665)
}
