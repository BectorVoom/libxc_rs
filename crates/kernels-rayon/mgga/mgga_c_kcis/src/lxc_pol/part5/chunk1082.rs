//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1082/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1082(t1747: f64, t2840: f64, t1017: f64, t86: f64, t4557: f64, t6700: f64, t922: f64, t3210: f64, t3200: f64, t1646: f64, t1670: f64, t829: f64) -> (f64, f64, f64, f64) {
    let t18525 = t2840 * t1747;
    let t18527 = t86 * t1017 * t18525;
    let t18528 = t18527 * t4557;
    let t18530 = t6700 * t922;
    let t18531 = t3210 * t18530;
    let t18532 = t3200 * t18531;
    let t18534 = t1646 * t1670;
    let t18535 = t18534 * t829;
    (t18528, t18532, t18534, t18535)
}
