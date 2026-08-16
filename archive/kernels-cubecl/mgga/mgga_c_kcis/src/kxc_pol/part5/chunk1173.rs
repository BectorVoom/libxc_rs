//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1173/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1173<F: Float>(t19702: F, t9517: F, t3200: F, t6704: F, t922: F, t3210: F, t1773: F, t829: F, t4566: F, t13410: F, t4554: F, t14628: F, t4984: F) -> (F, F, F, F, F) {
    let t19703 = t9517 * t19702;
    let t19704 = t3200 * t19703;
    let t19706 = t6704 * t922;
    let t19707 = t3210 * t19706;
    let t19708 = t3200 * t19707;
    let t19710 = t1773 * t829;
    let t19711 = t4566 * t19710;
    let t19712 = t13410 * t19711;
    let t19713 = t4554 * t19712;
    let t19715 = t14628 * t4984;
    (t19704, t19708, t19710, t19713, t19715)
}
