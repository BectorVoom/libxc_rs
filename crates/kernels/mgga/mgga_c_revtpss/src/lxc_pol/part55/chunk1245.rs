//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1245/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1245<F: Float>(t34251: F, t7003: F, t125563: F, t28196: F, t28286: F, t122275: F, t122279: F, t125570: F, t125573: F, t125576: F, t125578: F, t125580: F, t125582: F, t1903: F, t27841: F, t27903: F, t32250: F, t32677: F, t32690: F, t8706: F) -> (F, F, F) {
    let t128574 = F::new(2.0) * t34251 * t7003;
    let t128577 = F::new(2.0) * t28196 * t28286 * t125563;
    let t128594 = F::new(0.225875734067843736e-2) * t125570 - F::new(0.29749863367240808656e-2) * t125573 - F::new(0.29749863367240808656e-2) * t125576 + F::new(0.17347256376410398924e1) * t32690 * t27903 - F::new(0.17135921299530705785e1) * t8706 * t32250 * t32677 * t1903 + F::new(0.51405703062096148812e-1) * t122275 - F::new(0.28912093960683998208e-1) * t122279 - F::new(0.52041769129231196772e1) * t32690 * t27841 + F::new(0.7437465841810202164e-3) * t125578 + F::new(0.7437465841810202164e-3) * t125580 - F::new(0.74374658418102021639e-4) * t125582;
    (t128574, t128577, t128594)
}
