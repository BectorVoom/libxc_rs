//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1140/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1140<F: Float>(t14453: F, t4952: F, t991: F, t3040: F, t4966: F, t417: F, t13495: F, t4947: F, t1662: F, t2911: F, t9924: F, t13480: F, t4939: F) -> (F, F, F, F, F) {
    let t14454 = t14453 * t4952;
    let t14455 = t991 * t14454;
    let t14459 = t4966 * t3040;
    let t14460 = t417 * t14459;
    let t14463 = t4947 * t13495;
    let t14466 = t1662 * t2911;
    let t14467 = t9924 * t14466;
    let t14470 = t4939 * t13480;
    (t14455, t14460, t14463, t14467, t14470)
}
