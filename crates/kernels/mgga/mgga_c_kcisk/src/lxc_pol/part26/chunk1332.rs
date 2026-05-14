//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1332/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1332<F: Float>(t1333: F, t34783: F, t113888: F, t26871: F, t3482: F, t12827: F, t26950: F, t9461: F, t26954: F, t52483: F, t109846: F, t114361: F, t114368: F, t114377: F, t114395: F, t118898: F, t119194: F, t32087: F, t33373: F, t33377: F, t33384: F, t33389: F, t33400: F) -> (F, F, F, F, F) {
    let t119319 = t1333 * t34783;
    let t119322 = t3482 * t113888 * t26871;
    let t119336 = t12827 * t9461 * t26950;
    let t119339 = t52483 * t9461 * t26954;
    let t119341 = 0.46296296296296296297e-2 * t114361 - 0.55273148148148148147e-3 * t109846 - 0.7369753086419753086e-3 * t114368 - 0.16581944444444444444e-2 * t119319 - t114377 - 0.7369753086419753086e-3 * t119322 + 0.41666666666666666668e-1 * t32087 * t119194 + 0.46296296296296296296e-2 * t114395 - 0.13888888888888888889e-1 * t32087 * t118898 - 0.41666666666666666668e-1 * t33373 * t33389 - 0.24125000000000000001e-1 * t33377 * t33389 - 0.20833333333333333334e-1 * t33384 * t33400 + 0.73697530864197530861e-2 * t119336 + 0.11054629629629629629e-1 * t119339;
    (t119319, t119322, t119336, t119339, t119341)
}
