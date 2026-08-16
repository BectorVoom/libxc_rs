//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1173/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1173(t19702: f64, t9517: f64, t3200: f64, t6704: f64, t922: f64, t3210: f64, t1773: f64, t829: f64, t4566: f64, t13410: f64, t4554: f64, t14628: f64, t4984: f64) -> (f64, f64, f64, f64, f64) {
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
