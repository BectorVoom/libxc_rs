//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 585/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk585<F: Float>(t678: F, t7939: F, t2153: F, t275: F, t1347: F, t669: F, t2416: F, t7487: F, t2160: F, t2339: F, t638: F, t2323: F) -> (F, F, F, F, F, F) {
    let t7940 = t7939 * t678;
    let t7941 = F::new(0.19863479950205658386e-4) * t7940;
    let t7947 = t275 * t2153;
    let t7948 = F::new(2.0) * t7947;
    let t7949 = t1347 * t669;
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    (t7941, t7948, t7949, t8328, t8331, t8334)
}
