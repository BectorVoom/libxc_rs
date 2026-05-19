//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1253/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1253<F: Float>(t122275: F, t122279: F, t125570: F, t125573: F, t125576: F, t125578: F, t125580: F, t125582: F, t1903: F, t27841: F, t27903: F, t32250: F, t32677: F, t32690: F, t8706: F) -> F {
    let t128594 = F::cast_from(0.225875734067843736e-2_f64) * t125570 - F::cast_from(0.29749863367240808656e-2_f64) * t125573 - F::cast_from(0.29749863367240808656e-2_f64) * t125576 + F::cast_from(0.17347256376410398924e1_f64) * t32690 * t27903 - F::cast_from(0.17135921299530705785e1_f64) * t8706 * t32250 * t32677 * t1903 + F::cast_from(0.51405703062096148812e-1_f64) * t122275 - F::cast_from(0.28912093960683998208e-1_f64) * t122279 - F::cast_from(0.52041769129231196772e1_f64) * t32690 * t27841 + F::cast_from(0.7437465841810202164e-3_f64) * t125578 + F::cast_from(0.7437465841810202164e-3_f64) * t125580 - F::cast_from(0.74374658418102021639e-4_f64) * t125582;
    t128594
}
