//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 549/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk549(t1203: f64, t5036: f64, t1820: f64, t3325: f64, t3330: f64, t359: f64, t4772: f64, t376: f64, t1170: f64, t284: f64, t3463: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5037 = t5036 * t1203;
    let t5038 = t3325 * t1820;
    let t5039 = t1820 * t1203;
    let t5041 = 2.0_f64 * t3330 * t5039;
    let t5042 = t359 * t4772;
    let t5043 = t376 * t5042;
    let t5044 = t1170 * t5043;
    let t5046 = t3463 * t284;
    (t5037, t5038, t5039, t5041, t5043, t5044, t5046)
}
