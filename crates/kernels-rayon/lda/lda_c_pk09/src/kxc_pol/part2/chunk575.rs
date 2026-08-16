//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 575/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk575(t3194: f64, t4064: f64, t2974: f64, t1062: f64, t703: f64, t721: f64, t191: f64, t2971: f64, t1067: f64, t773: f64, t3743: f64, t932: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4065 = t4064 * t3194;
    let t4067 = t4064 * t2974;
    let t4069 = t703 * t1062;
    let t4070 = t4069 * t721;
    let t4072 = t191 * t2971;
    let t4077 = t773 * t1067;
    let t4085 = t932 * t3743;
    (t4065, t4067, t4070, t4072, t4077, t4085)
}
