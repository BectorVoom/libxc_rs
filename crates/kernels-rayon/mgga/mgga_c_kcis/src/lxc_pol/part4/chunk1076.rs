//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1076/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1076(t13484: f64, t13539: f64, t13576: f64, t13616: f64, t4852: f64, t829: f64, t1728: f64, t2635: f64, t3073: f64, t4670: f64, t1045: f64, t3096: f64, t4848: f64) -> (f64, f64, f64, f64, f64) {
    let t13618 = t13484 + t13539 + t13576 + t13616;
    let t13620 = t4852 * t829;
    let t13623 = t1728 * t2635;
    let t13626 = t3073 * t4670;
    let t13627 = t13626 * t1045;
    let t13630 = t4848 * t3096;
    (t13618, t13620, t13623, t13627, t13630)
}
