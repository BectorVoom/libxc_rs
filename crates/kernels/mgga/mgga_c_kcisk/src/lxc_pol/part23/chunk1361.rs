//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1361/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1361<F: Float>(t21499: F, t33459: F, t32019: F, t33451: F, t32022: F, t33593: F, t1308: F, t13437: F, t2158: F, t20233: F, t33427: F, t32087: F, t1319: F, t399: F, t19968: F, t32070: F) -> (F, F, F, F, F, F, F) {
    let t113947 = t33459 * t21499;
    let t113951 = 0.69444444444444444446e-2 * t32019 * t33451;
    let t113955 = 0.18518518518518518519e-1 * t32022 * t33593;
    let t113959 = t13437 * t2158 * t1308;
    let t113962 = t20233 * t33427;
    let t113963 = t32087 * t113962;
    let t113971 = t399 * t1319;
    let t113973 = t113971 * t19968 * t32070;
    (t113947, t113951, t113955, t113959, t113962, t113963, t113973)
}
