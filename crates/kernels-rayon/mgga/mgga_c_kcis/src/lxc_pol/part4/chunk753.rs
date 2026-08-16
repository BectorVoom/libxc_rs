//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 753/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk753(t169: f64, t4532: f64, t234: f64, t441: f64, t233: f64, t1295: f64, t1658: f64, t1877: f64, t911: f64, t1876: f64, t915: f64, t1071: f64, t359: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t4533 = piecewise3(t170, 0.0_f64, t4532);
    let t4534 = t234 * t4533;
    let t4535 = t4534 * t441;
    let t4536 = t233 * t4535;
    let t4538 = t1658 * t1295;
    let t4539 = t233 * t4538;
    let t4541 = t911 * t1877;
    let t4543 = t915 * t1876;
    let t4544 = t233 * t4543;
    let t4546 = t359 * t1071;
    (t4534, t4535, t4536, t4538, t4539, t4541, t4543, t4544, t4546)
}
