//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1166/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1166(t1092: f64, t19616: f64, t1767: f64, t4772: f64, t1022: f64, t3227: f64, t19550: f64, t19555: f64, t19559: f64, t19563: f64, t19566: f64, t19569: f64, t19573: f64, t19578: f64, t19582: f64, t19586: f64, t19591: f64, t19596: f64, t19601: f64, t19603: f64, t19607: f64, t19612: f64, t9529: f64) -> (f64, f64, f64, f64) {
    let t19617 = t1092 * t19616;
    let t19619 = t1767 * t4772;
    let t19620 = t1022 * t19619;
    let t19621 = t3227 * t19620;
    let t19622 = t1092 * t19621;
    let t19624 = 0.18424382716049382715e-2_f64 * t19550 - 0.22109259259259259259e-2_f64 * t19555 + 0.99491666666666666664e-2_f64 * t19559 - 0.13265555555555555555e-1_f64 * t19563 + 0.88437037037037037035e-2_f64 * t19566 + 0.55273148148148148147e-3_f64 * t9529 + 0.22109259259259259259e-2_f64 * t19569 - 0.22109259259259259258e-2_f64 * t19573 + 0.11054629629629629629e-2_f64 * t19578 - 0.73697530864197530861e-2_f64 * t19582 + 0.16581944444444444444e-2_f64 * t19586 + 0.73697530864197530862e-3_f64 * t19591 + 0.55273148148148148147e-3_f64 * t19596 - 0.16581944444444444444e-2_f64 * t19601 + 0.11054629629629629629e-2_f64 * t19603 - 0.44218518518518518517e-2_f64 * t19607 - 0.1492375e-1_f64 * t19612 + 0.33163888888888888888e-2_f64 * t19617 + 0.99491666666666666664e-2_f64 * t19622;
    (t19617, t19619, t19622, t19624)
}
