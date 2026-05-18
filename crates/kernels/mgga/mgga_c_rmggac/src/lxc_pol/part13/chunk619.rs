//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 619/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk619<F: Float>(t8141: F, t8158: F, t515: F, t235: F, t7579: F, t7678: F, t7680: F, t7683: F, t7685: F, t7688: F, t7692: F, t7697: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8159 = t8141 + t8158;
    let t8160 = t515 * t8159;
    let t8161 = t235 * t8160;
    let t8162 = F::new(0.19957069503106347607e-1) * t8161;
    let t8163 = F::new(0.5987120850931904282e-1) * t7579;
    let t8164 = F::new(0.85129199786595678799e-5) * t7678;
    let t8166 = F::new(0.5107751987195740728e-4) * t7680;
    let t8167 = F::new(0.2553875993597870364e-4) * t7683;
    let t8168 = F::new(0.1702583995731913576e-4) * t7685;
    let t8169 = F::new(0.85129199786595678799e-5) * t7688;
    let t8170 = F::new(0.212822999466489197e-4) * t7692;
    let t8171 = F::new(0.1064114997332445985e-4) * t7697;
    (t8159, t8160, t8162, t8163, t8164, t8166, t8167, t8168, t8169, t8170, t8171)
}
