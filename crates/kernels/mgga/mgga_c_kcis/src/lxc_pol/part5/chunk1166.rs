//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1166/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1166<F: Float>(t1092: F, t19616: F, t1767: F, t4772: F, t1022: F, t3227: F, t19550: F, t19555: F, t19559: F, t19563: F, t19566: F, t19569: F, t19573: F, t19578: F, t19582: F, t19586: F, t19591: F, t19596: F, t19601: F, t19603: F, t19607: F, t19612: F, t9529: F) -> (F, F, F, F) {
    let t19617 = t1092 * t19616;
    let t19619 = t1767 * t4772;
    let t19620 = t1022 * t19619;
    let t19621 = t3227 * t19620;
    let t19622 = t1092 * t19621;
    let t19624 = F::cast_from(0.18424382716049382715e-2_f64) * t19550 - F::cast_from(0.22109259259259259259e-2_f64) * t19555 + F::cast_from(0.99491666666666666664e-2_f64) * t19559 - F::cast_from(0.13265555555555555555e-1_f64) * t19563 + F::cast_from(0.88437037037037037035e-2_f64) * t19566 + F::cast_from(0.55273148148148148147e-3_f64) * t9529 + F::cast_from(0.22109259259259259259e-2_f64) * t19569 - F::cast_from(0.22109259259259259258e-2_f64) * t19573 + F::cast_from(0.11054629629629629629e-2_f64) * t19578 - F::cast_from(0.73697530864197530861e-2_f64) * t19582 + F::cast_from(0.16581944444444444444e-2_f64) * t19586 + F::cast_from(0.73697530864197530862e-3_f64) * t19591 + F::cast_from(0.55273148148148148147e-3_f64) * t19596 - F::cast_from(0.16581944444444444444e-2_f64) * t19601 + F::cast_from(0.11054629629629629629e-2_f64) * t19603 - F::cast_from(0.44218518518518518517e-2_f64) * t19607 - F::new(0.1492375e-1) * t19612 + F::cast_from(0.33163888888888888888e-2_f64) * t19617 + F::cast_from(0.99491666666666666664e-2_f64) * t19622;
    (t19617, t19619, t19622, t19624)
}
