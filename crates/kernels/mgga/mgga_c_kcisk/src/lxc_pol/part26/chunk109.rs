//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 109/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk109<F: Float>(t344: F, t347: F, t350: F, t355: F) -> (F, F, F, F) {
    let t365 = 1.0 + 0.278125e-1 * t344;
    let t370 = 0.51785e1 * t347 + 0.905775e0 * t344 + 0.1100325e0 * t350 + 0.248355e0 * t355;
    let t373 = 1.0 + 0.29608574643216675549e2 / t370;
    let t374 = f64::ln(t373);
    (t365, t370, t373, t374)
}
