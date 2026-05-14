//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1050/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1050<F: Float>(t1021: F, t19605: F, t1020: F, t1121: F, t6486: F, t1022: F, t9589: F, t1092: F, t1133: F, t1131: F, t3227: F, t1767: F, t4772: F, t19550: F, t19555: F, t19559: F, t19563: F, t19566: F, t19569: F, t19573: F, t19578: F, t19582: F, t19586: F, t19591: F, t19596: F, t19601: F, t19603: F, t9529: F) -> (F, F, F, F, F, F, F, F) {
    let t19606 = t1021 * t19605;
    let t19607 = t1020 * t19606;
    let t19609 = t6486 * t1121;
    let t19610 = t1022 * t19609;
    let t19611 = t9589 * t19610;
    let t19612 = t1092 * t19611;
    let t19614 = t6486 * t1133;
    let t19615 = t1131 * t19614;
    let t19616 = t3227 * t19615;
    let t19617 = t1092 * t19616;
    let t19619 = t1767 * t4772;
    let t19620 = t1022 * t19619;
    let t19621 = t3227 * t19620;
    let t19622 = t1092 * t19621;
    let t19624 = 0.18424382716049382715e-2 * t19550 - 0.22109259259259259259e-2 * t19555 + 0.99491666666666666664e-2 * t19559 - 0.13265555555555555555e-1 * t19563 + 0.88437037037037037035e-2 * t19566 + 0.55273148148148148147e-3 * t9529 + 0.22109259259259259259e-2 * t19569 - 0.22109259259259259258e-2 * t19573 + 0.11054629629629629629e-2 * t19578 - 0.73697530864197530861e-2 * t19582 + 0.16581944444444444444e-2 * t19586 + 0.73697530864197530862e-3 * t19591 + 0.55273148148148148147e-3 * t19596 - 0.16581944444444444444e-2 * t19601 + 0.11054629629629629629e-2 * t19603 - 0.44218518518518518517e-2 * t19607 - 0.1492375e-1 * t19612 + 0.33163888888888888888e-2 * t19617 + 0.99491666666666666664e-2 * t19622;
    (t19607, t19609, t19612, t19614, t19617, t19619, t19622, t19624)
}
