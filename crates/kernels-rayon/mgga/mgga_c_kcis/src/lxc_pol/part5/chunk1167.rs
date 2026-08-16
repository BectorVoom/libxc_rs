//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1167/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1167(t3178: f64, t6492: f64, t1092: f64, t1773: f64, t4772: f64, t1131: f64, t1096: f64, t1023: f64, t18463: f64, t1020: f64, t6689: f64, t922: f64) -> (f64, f64, f64, f64, f64) {
    let t19627 = t3178 * t6492;
    let t19628 = t1092 * t19627;
    let t19630 = t4772 * t1773;
    let t19631 = t1131 * t19630;
    let t19632 = t1096 * t19631;
    let t19633 = t1092 * t19632;
    let t19635 = t18463 * t1023;
    let t19636 = t1020 * t19635;
    let t19638 = t6689 * t922;
    (t19628, t19630, t19633, t19636, t19638)
}
