//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1245/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1245(t34251: f64, t7003: f64, t125563: f64, t28196: f64, t28286: f64, t122275: f64, t122279: f64, t125570: f64, t125573: f64, t125576: f64, t125578: f64, t125580: f64, t125582: f64, t1903: f64, t27841: f64, t27903: f64, t32250: f64, t32677: f64, t32690: f64, t8706: f64) -> (f64, f64, f64) {
    let t128574 = 2.0_f64 * t34251 * t7003;
    let t128577 = 2.0_f64 * t28196 * t28286 * t125563;
    let t128594 = 0.225875734067843736e-2_f64 * t125570 - 0.29749863367240808656e-2_f64 * t125573 - 0.29749863367240808656e-2_f64 * t125576 + 0.17347256376410398924e1_f64 * t32690 * t27903 - 0.17135921299530705785e1_f64 * t8706 * t32250 * t32677 * t1903 + 0.51405703062096148812e-1_f64 * t122275 - 0.28912093960683998208e-1_f64 * t122279 - 0.52041769129231196772e1_f64 * t32690 * t27841 + 0.7437465841810202164e-3_f64 * t125578 + 0.7437465841810202164e-3_f64 * t125580 - 0.74374658418102021639e-4_f64 * t125582;
    (t128574, t128577, t128594)
}
