//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 952/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk952<F: Float>(t13495: F, t4947: F, t1662: F, t2911: F, t9924: F, t13480: F, t4939: F, t2635: F, t4961: F, t2894: F, t1704: F, t2844: F) -> (F, F, F, F, F) {
    let t14463 = t4947 * t13495;
    let t14466 = t1662 * t2911;
    let t14467 = t9924 * t14466;
    let t14470 = t4939 * t13480;
    let t14473 = t4961 * t2635;
    let t14474 = t2894 * t14473;
    let t14477 = t1704 * t2844;
    (t14463, t14467, t14470, t14474, t14477)
}
